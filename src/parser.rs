use crate::data_model::{Condition, Query};


pub fn parse_query(query_str: &str) -> Result<Query, String> {
    let parts: Vec<&str> = query_str.trim().split_whitespace().collect();

    if parts.len() < 4 || parts[0].to_uppercase() != "SELECT" || parts[1] != "*" || parts[2].to_uppercase() != "FROM" {
        return Err("Invalid simple SELECT query syntax.".to_string());
    }

    let table_name = parts[3].to_string();
    let mut condition = None;

    // If the query contains a WHERE clause
    if parts.len() > 4 && parts[4].to_uppercase() == "WHERE" {
        if parts.len() < 8 {
            return Err("Invalid WHERE clause syntax.".to_string());
        }

        let column = parts[5].to_string();
        let operator = parts[6].to_string();
        // Join remaining parts for value by handling spaces and quotation marks
        let value = parts[7..].join(" ").trim_matches(';').trim_matches('\'').trim_matches('"').to_string();
        
        condition = Some(Condition {
            column,
            operator,
            value,
        });
    }

    Ok(Query {
        table_name,
        condition,
    })
}