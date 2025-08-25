use std::collections::HashMap;

// Represents a row in our database
// A HasMap is great for storing pairs (colunm_name, value)
#[derive(Debug, Clone)]
pub struct Row {
  pub data: HashMap<String, String>
}

impl Row {
    pub fn new(data: HashMap<String, String>) -> Self {
        Row { data }
    }

    // Added a method to retrieve a value securely
    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

// Represents our database, a collection of rows
pub struct Database {
    pub tables: HashMap<String, Vec<Row>>,
}

// Represents the condition of the query
#[derive(Debug, PartialEq)]
pub struct Condition {
    pub column: String,
    pub operator: String,
    pub value: String,
}

// Represents the parsed query
#[derive(Debug)]
pub struct Query {
    pub table_name: String,
    pub condition: Option<Condition>,
}