use std::{collections::HashMap, io::{self, Write}};

use crate::{
    data_model::{Database, Row, Table, Value},
    execute::execute_query,
    parser::parse_query,
};

mod data_model;
mod execute;
mod parser;

fn main() {
    // 1. Initialize db
    let mut db = Database {
        tables: HashMap::new(),
    };

    let mut users_table = Table {
        rows: Vec::new(),
        indexes: HashMap::new(),
    };

    // Data for db
    let mut row1_data = HashMap::new();
    row1_data.insert("name".to_string(), Value::Text("Alice".to_string()));
    row1_data.insert("age".to_string(), Value::Integer(30));
    let row1 = Row { data: row1_data };

    let mut row2_data = HashMap::new();
    row2_data.insert("name".to_string(), Value::Text("Bob".to_string()));
    row2_data.insert("age".to_string(), Value::Integer(45));
    let row2 = Row { data: row2_data };

    let mut row3_data = HashMap::new();
    row3_data.insert("name".to_string(), Value::Text("Charlie".to_string()));
    row3_data.insert("age".to_string(), Value::Integer(25));
    let row3 = Row { data: row3_data };

    users_table.rows.push(row1.clone());
    users_table.rows.push(row2.clone());
    users_table.rows.push(row3.clone());

    // 2. Building the index for column "age"
    let index_column = "age".to_string();
    let mut age_index = HashMap::new();
    for (i, row) in users_table.rows.iter().enumerate() {
        if let Some(value) = row.data.get(&index_column) {
            age_index
                .entry(value.clone())
                .or_insert_with(Vec::new)
                .push(i);
        }
    }
    users_table.indexes.insert(index_column, age_index);

    // Adding the table to the db
    db.tables.insert("users".to_string(), users_table);

    // 3. Loop on the user queries
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();

        let query = query.trim().to_string();
        if query.to_uppercase() == "EXIT" {
            break;
        }

        match parse_query(&query) {
            Ok(parsed_query) => {
                println!("Parsed query : {:#?}", parsed_query);
                match execute_query(&parsed_query, &db) {
                    Ok(results) => {
                        println!("Results found: {} lines", results.len());
                        for row in results {
                            println!("{:#?}", row);
                        }
                    }
                    Err(e) => println!("Runtime error : {}", e),
                }
            }
            Err(e) => println!("Parsing error: {}", e),
        }
    }
}
