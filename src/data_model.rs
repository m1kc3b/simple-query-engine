use std::collections::{BTreeMap, HashMap};

/// Unique identifier for each row in a table.
/// Advantage:
/// - Avoids data copies (an integer is stored instead of an entire Row in the indexes).
/// - Easy to increment.
/// - Compact in memory (u64 = 8 bytes).
pub type RowId = u64;

/// Typed value stored in a column.
// In a "real" DB, we would have an optimized binary storage engine.
/// Here, we keep it simple but correct:
/// - Fast comparison for Integer (u64).
/// - String for Text (not optimized but readable).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Integer(i64),
    Text(String),
}

/// A row in a table:
/// - represented as a column -> value dictionary.
/// - flexible, but not the most compact (each key is duplicated as a String).
/// - we'll see later that a real engine prefers a "column-by-column" layout.
#[derive(Debug, Clone)]
pub struct Row {
    pub data: HashMap<String, Value>,
}

impl Row {
    /// Convenient access to one value per column.
    pub fn get_value(&self, column: &str) -> Option<&Value> {
        self.data.get(column)
    }
}

/// Generic index based on a BTree:
/// - The key is a value (e.g., age = 30).
/// - Each key points to a list of RowIds.
/// Why BTree?
/// - Allows you to perform range queries (>, <, >=, <=).
/// - Keeps keys sorted in memory.
/// - Insertion/deletion in O(log n).
#[derive(Debug, Clone)]
pub struct Index {
    pub map: BTreeMap<Value, Vec<RowId>>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, value: Value, row_id: RowId) {
        self.map.entry(value).or_insert_with(Vec::new).push(row_id);
    }

    pub fn remove(&mut self, value: &Value, row_id: RowId) {
        if let Some(bucket) = self.map.get_mut(value) {
            bucket.retain(|&id| id != row_id);
            if bucket.is_empty() {
                self.map.remove(value);
            }
        }
    }
}

/// A table contains:
/// - an auto-incrementing counter for RowId.
/// - all rows (RowId -> Row).
/// - optional indexes (column -> Index).
#[derive(Debug, Clone)]
pub struct Table {
    pub rows: HashMap<RowId, Row>,
    pub indexes: HashMap<String, Index>,
    pub next_row_id: RowId,
}

impl Table {
    pub fn new() -> Self {
        Table {
            rows: HashMap::new(),
            indexes: HashMap::new(),
            next_row_id: 0,
        }
    }

    /// Insert a row:
    /// - Assigns a unique RowId.
    /// - Stores the row in `rows`.
    /// - Updates all corresponding indexes.
    pub fn insert(&mut self, row: Row) -> RowId {
        let row_id = self.next_row_id;
        self.next_row_id += 1;

        // insert into rows
        self.rows.insert(row_id, row.clone());

        // update the indexes
        for (col, idx) in self.indexes.iter_mut() {
            if let Some(val) = row.get_value(col) {
                idx.insert(val.clone(), row_id);
            }
        }

        row_id
    }

    /// Create an index on a column.
    pub fn create_index(&mut self, column: &str) {
        let mut index = Index::new();
        for (row_id, row) in &self.rows {
            if let Some(val) = row.get_value(column) {
                index.insert(val.clone(), *row_id);
            }
        }
        self.indexes.insert(column.to_string(), index);
    }
}

/// Database = collection of tables.
#[derive(Debug, Clone)]
pub struct Database {
    pub tables: HashMap<String, Table>,
}
