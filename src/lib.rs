#[macro_use]
extern crate cpython;

use cpython::{exc, PyClone, PyErr, PyResult, PyString, Python, ToPyObject};
use std::io::prelude::*;
extern crate keepass;

use keepass::{Database, NodeRef};
use std::error;
use std::fs::File;

fn get_meta_and_entries(
    _py: Python,
    db_path: String,
    password: Option<String>,
    keyfile: Option<String>,
) -> PyResult<(Meta, Vec<Group>, Vec<Entry>)> {
    let res = _get_meta_and_entries(_py, db_path, password, keyfile);
    match res {
        Err(e) => Err(PyErr::new::<exc::IOError, _>(_py, e.to_string())),
        Ok(v) => Ok(v),
    }
}

fn version(_py: Python, db_path: String) -> PyResult<u16> {
    let res = _version(db_path);
    match res {
        Err(e) => Err(PyErr::new::<exc::IOError, _>(_py, e.to_string())),
        Ok(v) => Ok(v),
    }
}

fn _version(db_path: String) -> Result<u16, Box<dyn error::Error>> {
    let mut data = Vec::new();
    let mut source = File::open(db_path)?;
    source.read_to_end(&mut data)?;

    let (_version, file_major_version, _file_minor_version) =
        keepass::parse::get_kdbx_version(data.as_ref())?;
    Ok(file_major_version)
}

py_class!(class Entry |py| {
    data _group: Group;
    data _title: String;
    data _url: String;
    data _username: String;
    data _password: String;
    data _notes: String;
    @property def group(&self) -> PyResult<Group> {
        Ok(self._group(py).clone_ref(py))
    }
    @property def title(&self) -> PyResult<String> {
        Ok(self._title(py).clone())
    }
    @property def url(&self) -> PyResult<String> {
        Ok(self._url(py).clone())
    }
    @property def username(&self) -> PyResult<String> {
        Ok(self._username(py).clone())
    }
    @property def password(&self) -> PyResult<String> {
        Ok(self._password(py).clone())
    }
    @property def notes(&self) -> PyResult<String> {
        Ok(self._notes(py).clone())
    }
});

py_class!(class Meta |py| {
    data _recycle_bin_uuid: String;
    @property def recycle_bin_uuid(&self) -> PyResult<String> {
        Ok(self._recycle_bin_uuid(py).to_string())
    }
    def __str__(&self) -> PyResult<String> {
        Ok(format!("<Meta {}>", self._recycle_bin_uuid(py)))
    }

});

py_class!(class Group |py| {
    data _name: String;
    data _uuid: String;

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
) -> Result<(Meta, Vec<Group>, Vec<Entry>), Box<dyn error::Error>> {
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

    let meta = Meta::create_instance(_py, db.meta.recyclebin_uuid.clone()).unwrap();
    let mut entries = Vec::new();
    let mut groups = Vec::new();

    let root_group =
        Group::create_instance(_py, db.root.name.clone(), db.root.uuid.clone()).unwrap();
    flatten_children(
        _py,
        db.root.children.iter().map(|n| n.into()).collect(),
        &mut entries,
        root_group,
    );

    let obj = Group::create_instance(_py, db.root.name, db.root.uuid).unwrap();
    groups.push(obj);

    flatten_groups(
        _py,
        db.root.children.iter().map(|n| n.into()).collect(),
        &mut groups,
    );

    Ok((meta, groups, entries))
}

fn flatten_groups(py: Python, nodes: Vec<NodeRef>, group_map: &mut Vec<Group>) -> () {
    for node in nodes {
        match node {
            NodeRef::Group(g) => {
                let obj = Group::create_instance(py, g.name.clone(), g.uuid.clone()).unwrap();
                group_map.push(obj);
                flatten_groups(py, g.children.iter().map(|n| n.into()).collect(), group_map);
            }
            _ => {}
        }
    }
}
fn flatten_children(py: Python, nodes: Vec<NodeRef>, entries: &mut Vec<Entry>, group: Group) -> () {
    for node in nodes {
        match node {
            NodeRef::Group(g) => {
                let _g = Group::create_instance(py, g.name.clone(), g.uuid.clone()).unwrap();
                flatten_children(
                    py,
                    g.children.iter().map(|n| n.into()).collect(),
                    entries,
                    _g,
                );
            }
            NodeRef::Entry(e) => {
                let _e = Entry::create_instance(
                    py,
                    group.clone_ref(py),
                    e.get_title().unwrap_or("").to_string(),
                    e.get("URL").unwrap_or("").to_string(),
                    e.get_username().unwrap_or("").to_string(),
                    e.get_password().unwrap_or("").to_string(),
                    e.get("Notes").unwrap_or("").to_string(),
                )
                .unwrap();
                entries.push(_e);
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
    m.add(py, "get_db_version", py_fn!(py, version(db_path: String,)))?;
    Ok(())
});
