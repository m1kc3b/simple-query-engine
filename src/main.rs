use std::{collections::{HashMap, BTreeMap}, io::{self, Write}};

use crate::{
    data_model::{Database, Row, Table, Value, RowId, Index},
    execute::execute_query,
    parser::parse_query,
};

mod data_model;
mod execute;
mod parser;

fn main() {
    // 1. Initialisation de la base de données
    let mut db = Database {
        tables: HashMap::new(),
    };

    // Création de la table "users" avec HashMap<usize, Row> pour les lignes
    let mut users_table = Table::new();

    // 2. Ajout des lignes dans la table
    let rows_data = vec![
        ("Alice", 30),
        ("Bob", 45),
        ("Charlie", 25),
    ];

    for (i, (name, age)) in rows_data.iter().enumerate() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::Text(name.to_string()));
        data.insert("age".to_string(), Value::Integer(*age));
        let row = Row { data };
        users_table.rows.insert(i as RowId, row);
    }

    // 3. Construction de l'index pour la colonne "age"
    let mut age_index_map: BTreeMap<Value, Vec<RowId>> = BTreeMap::new();
    for (&row_id, row) in users_table.rows.iter() {
        if let Some(value) = row.get_value("age") {
            age_index_map.entry(value.clone())
                .or_insert_with(Vec::new)
                .push(row_id);
        }
    }

    // On encapsule dans la struct Index
    users_table.indexes.insert(
        "age".to_string(),
        Index { map: age_index_map },
    );

    // 4. Ajout de la table dans la base
    db.tables.insert("users".to_string(), users_table);

    // 5. Boucle principale pour lire les requêtes de l'utilisateur
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();

        let query = query.trim().to_string();
        if query.to_uppercase() == "EXIT" {
            break;
        }

        // Parsing de la requête
        match parse_query(&query) {
            Ok(parsed_query) => {
                println!("Parsed query : {:#?}", parsed_query);

                // Exécution de la requête
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
