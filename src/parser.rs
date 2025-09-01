use crate::data_model::Value;

/// Representation of operators in conditions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Eq,    // =
    Neq,   // !=
    Gt,    // >
    Lt,    // <
    Gte,   // >=
    Lte,   // <=
}

/// WHERE condition
#[derive(Debug, Clone)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub value: Value,
}

/// Simplified SQL query
#[derive(Debug, Clone)]
pub struct Query {
    pub table_name: String,
    pub condition: Option<Condition>,
}

/// Function to parse a value from a string
fn parse_value(token: &str) -> Result<Value, String> {
    if let Ok(int_val) = token.parse::<i64>() {
        Ok(Value::Integer(int_val))
    } else {
        // Delete quotes if present
        let trimmed = token.trim_matches('"').trim_matches('\'');
        Ok(Value::Text(trimmed.to_string()))
    }
}

/// Main parsing function
pub fn parse_query(sql: &str) -> Result<Query, String> {
    let tokens: Vec<&str> = sql.split_whitespace().collect();

    // Minimalist verification
    if tokens.len() < 4 {
        return Err("Query too short".to_string());
    }
    if tokens[0].to_uppercase() != "SELECT" || tokens[1] != "*" || tokens[2].to_uppercase() != "FROM" {
        return Err("Invalid SELECT syntax".to_string());
    }

    let table_name = tokens[3].to_string();

    // Managing the WHERE clause
    if tokens.len() > 4 && tokens[4].to_uppercase() == "WHERE" {
        if tokens.len() < 8 {
            return Err("Incomplete WHERE clause".to_string());
        }
        let column = tokens[5].to_string();
        let op_str = tokens[6];
        let value_str = tokens[7];

        let operator = match op_str {
            "="  => Operator::Eq,
            "!=" => Operator::Neq,
            ">"  => Operator::Gt,
            "<"  => Operator::Lt,
            ">=" => Operator::Gte,
            "<=" => Operator::Lte,
            _ => return Err(format!("Unknown operator '{}'", op_str)),
        };

        let value = parse_value(value_str)?;

        Ok(Query {
            table_name,
            condition: Some(Condition { column, operator, value }),
        })
    } else {
        // No WHERE
        Ok(Query { table_name, condition: None })
    }
}
