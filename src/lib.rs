//! IMAP Filter library of functions
//#![warn(missing_docs)]

#[macro_use]
extern crate clap;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::result::Result;
use std::fs;
use std::path::Path;
use mlua::prelude::*;

pub mod command;
pub mod dsl;

const SIMPLE: &str = "example/simple.lua";
const LUA_HELPER: &str = r#"require "lua/imap-filter" "#;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum AuthType {
    Login,
    Plain,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Login {
    NamePassword((String, String)),
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Server {
    URL((String, u32)), // URI and port
}

/// Key for the struct
pub trait Key {
    fn key(&self) -> String;
}
    
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Account {
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
pub struct Filter {
    name: String,
}

impl Key for Filter {
    fn key(&self) -> String { self.name.clone() }
}

pub struct ImapFilterOperation {
    lua: Option<Lua>,
    script_path: String,
    im_require_path: String,
    accounts: HashMap<String, Account>,
    filters: HashMap<String, Filter>,
}

impl ImapFilterOperation {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ImapFilterOperation, &'static str> {
        let mut imf = ImapFilterOperation{
            lua: None,
            script_path: "".to_string(),
            im_require_path: "".to_string(),
            accounts: HashMap::new(),
            filters: HashMap::new()
        };
        match imf.load_configuration(path) {
            Ok(lua) => Ok(imf),
            Err(s) => Err(s)
        }
    }

    /// Used for test, do not call
    fn _lua(&mut self) -> &Lua {
        self.lua.as_ref().unwrap()
    }

    /// TODO: Currently this will panic if the config file does not
    /// TODO: exist. Handle this properly!!!!
    fn load_configuration<P: AsRef<Path>>(&mut self, path: P) -> Result<&Lua, &'static str> {
        match fs::read_to_string(path) {
            Ok(emf) => {
                self.lua = Some(Lua::new());
                let mut lua = &self.lua.as_ref().unwrap();

                dsl::setup_dsl(lua)
                    .load(LUA_HELPER)
                    .exec()
                    .unwrap(); // force an assertion if helper script is not found. TODO handle this case better.
                match lua.load(&emf).exec() {
                    Ok(_) => {
                        Ok(&lua)
                    },
                    Err(e) => {
                        print!("err: {:?}", e);
                        Err("Compilation error in filter script.")
                    }
                }
            },
            Err(s) => Err("File does not exist.")
        }
    }

    pub fn run(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_simple_sample() {
        match ImapFilterOperation::new(SIMPLE) {
            Ok(ifo) => {
                ifo.run();
            },
            Err(e) => {
                print!("err in LUA script: {:?}", e);
                assert!(false);
            }
        }
    }
}
