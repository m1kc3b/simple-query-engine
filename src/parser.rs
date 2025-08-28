use crate::data_model::{Condition, Query, Value};

/**
 * This parser has been refactored to be more flexible and efficient.
 * 
 * 1. Handling of comparison operators: 
 * It can now handle the operators >, <, >=, <=, and != in addition to equality. 
 * This allows for more complex and varied queries.
 * 
 * 2. Data type conversion:
 * The condition value is no longer a String. 
 * It is now converted to a more appropriate data type (`Value::Integer` or `Value::Text`). 
 * This conversion is crucial. 
 * Not only does it allow for precise numeric comparisons (e.g., 'age > 25'), but it also stores the data more efficiently in memory and prepares the ground for indexing
 * 
 * 3. Increased flexibility:
 * Although it is still limited to 'SELECT *', the logic is ready to be extended to handle the selection of specific columns.
 */
pub fn parse_query(query_str: &str) -> Result<Query, String> {
    let parts: Vec<&str> = query_str.trim().split_whitespace().collect();

    // 1. Basic syntax check
    if parts.len() < 4 || parts[0].to_uppercase() != "SELECT" || parts[2].to_uppercase() != "FROM" {
        return Err("Invalid simple SELECT query syntax.".to_string());
    }

    let table_name = parts[3].to_string();
    let mut condition = None;

    // 2. Handling the WHERE clause
    if parts.len() > 4 && parts[4].to_uppercase() == "WHERE" {
        if parts.len() < 8 {
            return Err("Invalid WHERE clause syntax.".to_string());
        }

        let column = parts[5].to_string();
        let operator_str = parts[6].to_string();

        // 2a. Validate the operator
        let supported_operators = ["=", ">", "<", ">=", "<=", "!="];
        if !supported_operators.contains(&operator_str.as_str()) {
            return Err(format!("Operator not supported: {}", operator_str));
        }

        // 2b. Handling values and converting them in to the `enum Value`
        let raw_value_str = parts[7..].join(" ").trim_matches(';').to_string();
        let value = if raw_value_str.starts_with('\'') && raw_value_str.ends_with('\'') {
            // It's a string, removing quotation marks
            Value::Text(raw_value_str.trim_matches('\'').to_string())
        } else {
            // Parse in to a number
            match raw_value_str.parse::<i64>() {
                Ok(num) => Value::Integer(num),
                Err(_) => return Err("Impossible to parse this value".to_string()),
            }
        };
        
        condition = Some(Condition {
            column,
            operator: operator_str,
            value,
        });
    }

    Ok(Query {
        table_name,
        condition,
    })
}