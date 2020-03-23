//! imap library of functions

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Debug)]
enum AuthType {
    Login,
    Plain,
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Login {
    NamePassword((String, String)),
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Server {
    URL((String, u32)), // URI and port
}

/// Key for the struct
trait Key {
    fn key(&self) -> String;
}
    
#[derive(Hash, Eq, PartialEq, Debug)]
struct Account {
    name: String,
    login: Login,
    server: Server,
    ssl: bool,
    auth: AuthType,
}

impl Key for Account {
    fn key(&self) -> String { self.name.clone() }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Filter {
    name: String,
}

impl Key for Filter {
    fn key(&self) -> String { self.name.clone() }
}

struct ImapFilterOperation {
    accounts: HashMap<String, Account>,
    filters: HashMap<String, Filter>
}


