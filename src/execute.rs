use crate::data_model::{Database, Query, Row, Value};

/**
 * The execution engine has been refactored to drastically improve query performance.
 *
 * 1. Optimized execution strategy:
 * The engine no longer simply scans the entire table.
 * It first checks if an index exists on the column in the condition (`WHERE`).
 *
 * 2. Use of indexes:
 * If an index is found, the engine uses it to directly and very quickly access the relevant rows.
 * This avoids the costly "full table scan" and significantly reduces latency.
 *
 * 3. Fallback handling:
 * If no index is available for the query column, the engine falls back to the full table scan method.
 * This ensures that the query always works, even if it is not optimized.
 */
pub fn execute_query(query: &Query, db: &Database) -> Result<Vec<Row>, String> {
    // 1. Retrieving table by name
    let table = db
        .tables
        .get(&query.table_name)
        .ok_or_else(|| format!("Table '{}' not found.", query.table_name))?;

    // 2. Initializing the results list
    let mut results = Vec::new();

    // 3. Handling the WHERE clause
    if let Some(condition) = &query.condition {
        // Checking if an index exists for the condition
        if let Some(index) = table.indexes.get(&condition.column) {
            // Searching for the value in our index
            if let Some(row_indices) = index.get(&condition.value) {
                // For each row index found, the corresponding row is retrieved
                for &row_index in row_indices {
                    if let Some(row) = table.rows.get(row_index) {
                        results.push(row.clone());
                    }
                }
            }
        } else {
            // No index. We must perform a full table scan.
            // The current logic from the execute.rs file will be reused here.
            // We scan each line and check the condition.
            for row in &table.rows {
                if let Some(row_value) = row.get_value(&condition.column) {
                    let row_matches = match condition.operator.as_str() {
                        "=" => row_value == &condition.value,
                        "!=" => row_value != &condition.value,
                        ">" => {
                            if let (Value::Integer(row_num), Value::Integer(cond_num)) =
                                (row_value, &condition.value)
                            {
                                row_num > cond_num
                            } else {
                                false
                            }
                        }
                        "<" => {
                            if let (Value::Integer(row_num), Value::Integer(cond_num)) =
                                (row_value, &condition.value)
                            {
                                row_num < cond_num
                            } else {
                                false
                            }
                        }
                        ">=" => {
                            if let (Value::Integer(row_num), Value::Integer(cond_num)) =
                                (row_value, &condition.value)
                            {
                                row_num >= cond_num
                            } else {
                                false
                            }
                        }
                        "<=" => {
                            if let (Value::Integer(row_num), Value::Integer(cond_num)) =
                                (row_value, &condition.value)
                            {
                                row_num <= cond_num
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    if row_matches {
                        results.push(row.clone());
                    } else {
                        continue;
                    }
                }
            }
        }
    } else {
        // No condition, we return all the rows
        results = table.rows.clone();
    }

    Ok(results)
}
