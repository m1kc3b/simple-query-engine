use std::collections::HashMap;

use crate::{data_model::{Database, Row}, execute::execute_query, parser::parse_query};

mod data_model;
mod parser;
mod execute;


fn main() {
    let mut db = Database { tables: HashMap::new() };

    let users_table = vec![
        Row::new(HashMap::from([
            ("id".to_string(), "1".to_string()),
            ("name".to_string(), "Alice".to_string()),
            ("age".to_string(), "30".to_string()),
        ])),
        Row::new(HashMap::from([
            ("id".to_string(), "2".to_string()),
            ("name".to_string(), "Bob".to_string()),
            ("age".to_string(), "25".to_string()),
        ])),
    ];

    db.tables.insert("users".to_string(), users_table);

    let query_str = "SELECT * FROM users WHERE name = 'Bob';";

    match parse_query(query_str) {
        Ok(query) => {
            println!("Parsed query: {:#?}", query);

            match execute_query(&query, &db) {
                Ok(results) => {
                    println!("Result of the query:");
                    for row in results {
                        println!("{:#?}", row);
                    }
                },
                Err(e) => println!("Execution error: {}", e),
            }
        },
        Err(e) => println!("Parsing Error: {}", e),
    }
}
