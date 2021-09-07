from typing import List, Tuple

class Group:
    name: str
    uuid: str

class Meta:
    recycle_bin_uuid: str

class Entry:
    group: Group
    title: str
    url: str
    username: str
    password: str
    notes: str

def get_meta_and_entries(db_path: str, password: str = None, keyfile: str = None) -> Tuple[Meta, List[Group], List[Entry]]: ...
def get_db_version(db_path: str) -> int: ...
