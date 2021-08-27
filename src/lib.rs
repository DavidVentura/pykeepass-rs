#[macro_use]
extern crate cpython;

use cpython::{exc, PyErr, PyResult, Python};
use std::io::prelude::*;
extern crate keepass;

use keepass::{Database, NodeRef};
use std::collections::HashMap;
use std::error;
use std::fs::File;

// create_exception!(pykeepass_rs, IncorrectKey, PyException);

fn get_all_entries(
    _py: Python,
    db_path: String,
    password: Option<String>,
    keyfile: Option<String>,
) -> PyResult<Vec<HashMap<String, String>>> {
    let res = _get_all_entries(db_path, password, keyfile);
    match res {
        Err(e) => Err(PyErr::new::<exc::IOError, _>(_py, e.to_string())),
        Ok(v) => Ok(v),
    }
}

fn _get_all_entries(
    db_path: String,
    password: Option<String>,
    keyfile: Option<String>,
) -> Result<Vec<HashMap<String, String>>, Box<dyn error::Error>> {
    let _db_path = std::path::Path::new(&db_path);
    let mut f;
    let p;
    let key_file = match keyfile {
        None => None,
        Some(k) => {
            p = std::path::Path::new(&k);
            f = File::open(p)?;
            Some(&mut f as &mut dyn Read)
        }
    };

    // Open KeePass database
    let db = Database::open(&mut File::open(db_path)?, password.as_deref(), key_file)?;

    // Iterate over all Groups and Nodes

    let mut ret = Vec::new();
    let mut group_name: String = "Root".to_string();
    for node in &db.root {
        match node {
            NodeRef::Group(g) => {
                group_name = g.name.clone();
            }
            NodeRef::Entry(e) => {
                //ret.push(e.fields);
                //should be able to push bytes always?

                let mut entry = HashMap::new();
                entry.insert(
                    "password".to_string(),
                    e.get_password().unwrap_or("").to_string(),
                );
                entry.insert(
                    "notes".to_string(),
                    e.get("Notes").unwrap_or("").to_string(),
                );
                entry.insert("title".to_string(), e.get_title().unwrap_or("").to_string());
                entry.insert(
                    "username".to_string(),
                    e.get_username().unwrap_or("").to_string(),
                );
                entry.insert("url".to_string(), e.get("URL").unwrap_or("").to_string());
                entry.insert("group".to_string(), group_name.clone());
                ret.push(entry);
            }
        }
    }

    Ok(ret)
}

py_module_initializer!(pykeepass_rs, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(
        py,
        "get_all_entries",
        py_fn!(
            py,
            get_all_entries(
                db_path: String,
                password: Option<String>,
                keyfile: Option<String>
            )
        ),
    )?;
    Ok(())
});
