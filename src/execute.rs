use crate::data_model::{Database, Row, Value};
use crate::parser::{Condition, Operator};

/// Executes a query on the database
///
/// Improvements:
/// 1. Use of indexes (BTreeMap) to speed up comparative queries.
/// 2. Full scan only if no index or operator not supported by the index.
/// 3. Uniform RowId (usize) for consistency with HashMap/Vec and to avoid conversions.
/// 4. Full operator support: =, !=, >, >=, <, <=
/// 5. Always returns a Vec<Row>, compatible with display or subsequent use.
pub fn execute_query(query: &crate::parser::Query, db: &Database) -> Result<Vec<Row>, String> {
    // 1. Retrieve the table from the database
    // If the table doesn't exist, an error is returned.
    let table = db
        .tables
        .get(&query.table_name)
        .ok_or_else(|| format!("Table '{}' not found.", query.table_name))?;

    // 2. Initialization of the final result
    let mut results = Vec::new();

    // 3. If the query has a WHERE condition
    if let Some(cond) = &query.condition {
        if let Some(index) = table.indexes.get(&cond.column) {
            // 3a. If an index exists for the column, use it to speed up the search.
            // Index = BTreeMap<Value, Vec<RowId>>
            let row_ids: Vec<usize> = match cond.operator {
                // Equality: direct retrieval in the BTreeMap
                Operator::Eq => index
                    .map
                    .get(&cond.value)
                    .cloned()
                    .unwrap_or_default(),// if the value does not exist, returns an empty Vec
                
                // Greater than / Greater than or equal to / Less than / Less than or equal to
                // Use BTreeMap::range to avoid looping through all rows
                Operator::Gt => index
                    .map
                    .range((
                        std::ops::Bound::Excluded(&cond.value),
                        std::ops::Bound::Unbounded,
                    ))
                    .flat_map(|(_, ids)| ids.clone())
                    .collect(),

                Operator::Gte => index
                    .map
                    .range((
                        std::ops::Bound::Included(&cond.value),
                        std::ops::Bound::Unbounded,
                    ))
                    .flat_map(|(_, ids)| ids.clone())
                    .collect(),

                Operator::Lt => index
                    .map
                    .range((
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Excluded(&cond.value),
                    ))
                    .flat_map(|(_, ids)| ids.clone())
                    .collect(),

                Operator::Lte => index
                    .map
                    .range((
                        std::ops::Bound::Unbounded,
                        std::ops::Bound::Included(&cond.value),
                    ))
                    .flat_map(|(_, ids)| ids.clone())
                    .collect(),

                // Different: full scan required because BTreeMap does not allow a direct search for "!="
                Operator::Neq => full_scan(&table.rows, cond),
            };

            // 3b. Retrieving the corresponding rows via the RowId
            for row_id in row_ids {
                if let Some(row) = table.rows.get(&row_id) {
                    results.push(row.clone());
                }
            }
        } else {
            // 3c. No index for this column → full scan
            results = full_scan(&table.rows, cond)
                .iter()
                .filter_map(|&id| table.rows.get(&id).cloned())
                .collect();
        }
    } else {
        // 4. No WHERE condition → return all rows
        // .values() retrieves all Rows, .cloned() to get the property
        results = table.rows.values().cloned().collect();
    }

    Ok(results)
}

/// Full scan fallback function
///
/// Scans all rows and returns the RowIds that match the condition.
/// Required for:
/// - Columns without indexes
/// - != operator
fn full_scan(rows: &std::collections::HashMap<usize, Row>, cond: &Condition) -> Vec<usize> {
    let mut ids = Vec::new();

    // We iterate over all the lines
    for (&row_id, row) in rows.iter() {
        if let Some(row_value) = row.get_value(&cond.column) {
            // Comparison according to the operator
            let matches = match cond.operator {
                Operator::Eq => row_value == &cond.value,
                Operator::Neq => row_value != &cond.value,
                Operator::Gt => match (row_value, &cond.value) {
                    (Value::Integer(a), Value::Integer(b)) => a > b,
                    _ => false,
                },
                Operator::Gte => match (row_value, &cond.value) {
                    (Value::Integer(a), Value::Integer(b)) => a >= b,
                    _ => false,
                },
                Operator::Lt => match (row_value, &cond.value) {
                    (Value::Integer(a), Value::Integer(b)) => a < b,
                    _ => false,
                },
                Operator::Lte => match (row_value, &cond.value) {
                    (Value::Integer(a), Value::Integer(b)) => a <= b,
                    _ => false,
                },
            };

            // If the row matches the condition, we keep its RowId
            if matches {
                ids.push(row_id);
            }
        }
    }

    ids
}
