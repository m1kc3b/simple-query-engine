use std::collections::HashMap;

/**
 * Using this data type is a crucial step for optimization. 
 * It allows us to store data more efficiently, using less memory than if everything were in String. 
 * In addition, it makes data comparisons more reliable. 
 * It is simpler and safer to compare two integers than two strings representing numbers.
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]  
pub enum Value {
    Interger(i64),
    Text(String),
}

// Represents a row in our database
// A HasMap is great for storing pairs (colunm_name, value)
#[derive(Debug, Clone)]
pub struct Row {
  pub data: HashMap<String, Value>
}

impl Row {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Row { data }
    }

    // Added a method to retrieve a value securely
    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

/**
 * By encapsulating rows and indexes in a single structure, you create a cleaner and more logical design. 
 * A table has rows, and it also has indexes for those rows. 
 * This sets the stage for our execution engine to easily find the appropriate indexes for a given query.
 */
pub struct Table {
    pub rows: Vec<Row>,
    pub indexes: HashMap<String, HashMap<Value, Vec<usize>>>,
}

// Represents our database, a collection of rows
pub struct Database {
    pub tables: HashMap<String, Table>,
}

// Represents the condition of the query
#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub column: String,
    pub operator: String,
    pub value: Value,
}

// Represents the parsed query
#[derive(Debug)]
pub struct Query {
    pub table_name: String,
    pub condition: Option<Condition>,
}