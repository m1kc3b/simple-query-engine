use crate::data_model::{Database, Query, Row};


pub fn execute_query(query: &Query, db: &Database) -> Result<Vec<Row>, String> {
    // Check if table exists
    let table = db.tables.get(&query.table_name)
        .ok_or_else(|| format!("Table '{}' non trouvée.", query.table_name))?;

    let mut results = Vec::new();

    // Iterates over each row of the table
    for row in table {
        let mut row_matches = true;

        // If the query has a WHERE condition
        if let Some(condition) = &query.condition {
            // Checks if the row contains the condition column
            if let Some(row_value) = row.get_value(&condition.column) {
                // Executes the condition (for now, only equality)
                if condition.operator == "=" {
                    if row_value != &condition.value {
                        row_matches = false;
                    }
                } else {
                    return Err(format!("Opérateur non supporté : {}", condition.operator));
                }
            } else {
                // If the column does not exist, the row does not match
                row_matches = false;
            }
        }

        // If the row matches all conditions, add it to the results
        if row_matches {
            results.push(row.clone());
        }
    }

    Ok(results)
}