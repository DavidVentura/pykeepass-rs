use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::io::prelude::*;
extern crate keepass;

use keepass::{Database, NodeRef};
use std::collections::HashMap;
use std::fs::File;

create_exception!(pykeepass_rs, IncorrectKey, PyException);

#[pyfunction]
fn get_all_entries(
    db_path: String,
    password: Option<String>,
    key_path: Option<String>,
) -> PyResult<Vec<HashMap<String, String>>> {
    let _db_path = std::path::Path::new(&db_path);
    let mut f;
    let p;
    let key_file = match key_path {
        None => None,
        Some(k) => {
            p = std::path::Path::new(&k);
            f = File::open(p)?;
            Some(&mut f as &mut dyn Read)
        }
    };

    // Open KeePass database
    let db = Database::open(&mut File::open(db_path)?, password.as_deref(), key_file);
    if let Err(e) = db {
        return Err(IncorrectKey::new_err(e.to_string()));
    }

    let db = db.unwrap();

    // Iterate over all Groups and Nodes

    let mut ret = Vec::new();
    let mut group_name: String = "Root".to_string();
    for node in &db.root {
        match node {
            NodeRef::Group(g) => {
                group_name = g.name.clone();
            }
            NodeRef::Entry(e) => {
                let mut entry = HashMap::new();
                entry.insert(
                    "password".to_string(),
                    e.get_password().unwrap_or("").to_string(),
                );
                entry.insert(
                    "notes".to_string(),
                    e.get("notes").unwrap_or("").to_string(),
                );
                entry.insert("title".to_string(), e.get_title().unwrap_or("").to_string());
                entry.insert(
                    "username".to_string(),
                    e.get_username().unwrap_or("").to_string(),
                );
                entry.insert("url".to_string(), e.get("url").unwrap_or("").to_string());
                entry.insert("group".to_string(), group_name.clone());
                ret.push(entry);
            }
        }
    }

    Ok(ret)
}

#[pymodule]
fn pykeepass_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_all_entries, m)?)?;
    Ok(())
}
