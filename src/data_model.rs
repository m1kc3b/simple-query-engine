use std::collections::HashMap;

// Represents a row in our database
// A HasMap is great for storing pairs (colunm_name, value)
#[derive(Debug, Clone)]
pub struct Row(HashMap<String, String>);

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