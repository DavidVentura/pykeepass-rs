#[macro_use]
extern crate cpython;

use cpython::{exc, PyErr, PyResult, PyString, Python, ToPyObject};
use std::io::prelude::*;
extern crate keepass;

use keepass::{Database, NodeRef};
use std::collections::HashMap;
use std::error;
use std::fs::File;

fn get_meta_and_entries(
    _py: Python,
    db_path: String,
    password: Option<String>,
    keyfile: Option<String>,
) -> PyResult<(
    HashMap<String, String>,
    HashMap<String, Vec<HashMap<String, String>>>,
)> {
    let res = _get_meta_and_entries(_py, db_path, password, keyfile);
    match res {
        Err(e) => Err(PyErr::new::<exc::IOError, _>(_py, e.to_string())),
        Ok(v) => Ok(v),
    }
}

py_class!(class Group |py| {
    data _name: String;
    data _uuid: String;
    //data _entries: HashMap<String, String>;

    def __str__(&self) -> PyResult<impl ToPyObject<ObjectType=PyString>> {
        Ok(format!("<Group {}={}>", self._name(py), self._uuid(py)))
    }

    @property def uuid(&self) -> PyResult<String> {
        Ok(self._uuid(py).to_string())
    }


    @property def name(&self) -> PyResult<String> {
        Ok(self._name(py).to_string())
    }


});

fn _get_meta_and_entries(
    _py: Python,
    db_path: String,
    password: Option<String>,
    keyfile: Option<String>,
) -> Result<
    (
        HashMap<String, String>,
        HashMap<String, Vec<HashMap<String, String>>>,
    ),
    Box<dyn error::Error>,
> {
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

    let mut meta = HashMap::new();
    let mut entries = HashMap::new();

    entries.insert(db.root.name.clone(), Vec::new());

    meta.insert("recycle_bin_uuid".to_string(), db.meta.recyclebin_uuid);
    flatten_children(
        db.root.children.iter().map(|n| n.into()).collect(),
        &mut entries,
        db.root.name,
    );

    Ok((meta, entries))
}

fn flatten_children(
    nodes: Vec<NodeRef>,
    group_map: &mut HashMap<String, Vec<HashMap<String, String>>>,
    group_name: String,
) -> () {
    for node in nodes {
        match node {
            NodeRef::Group(g) => {
                group_map.insert(g.name.clone(), Vec::new());
                flatten_children(
                    g.children.iter().map(|n| n.into()).collect(),
                    group_map,
                    g.name.clone(),
                );
            }
            NodeRef::Entry(e) => {
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
                match group_map.get_mut(&group_name) {
                    Some(items) => items.push(entry),
                    None => panic!("Could not get any item for group name {}", group_name),
                }
            }
        }
    }
}

py_module_initializer!(pykeepass_rs, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(
        py,
        "get_meta_and_entries",
        py_fn!(
            py,
            get_meta_and_entries(
                db_path: String,
                password: Option<String>,
                keyfile: Option<String>
            )
        ),
    )?;
    Ok(())
});
