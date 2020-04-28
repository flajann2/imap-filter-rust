//! IMAP Filter library of functions
//#![warn(missing_docs)]

// TODO: remove the following beofre release
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]

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

pub type Details = Vec<ImapTypes>;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum AuthTypes {
    Plain,
    Login,
}

// this supercedes what's in the lib.rs, which will eventually be deleted.
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum MarkTypes {
    Seen,
    Unseen
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ImapTypes {
    Account(String, Details),
    Auth(AuthTypes),
    Login(Details),
    User(String),
    Password(String),
    Serv(String),
    TLS,
    SSL,
    Port(i32),

    Filter(String, Details),
    Search(String, Details),
    From(Vec<String>),
    To(Vec<String>),
    Cc(Vec<String>),
    Bcc(Vec<String>),
    Seen,
    Unseen,

    Mark(MarkTypes),
    Copy(Vec<String>),
    Move(Vec<String>),
    Delete,
}

pub struct ImapFilterOperation {
    lua: Option<Lua>,
    script_path: String,
    im_require_path: String,
    accounts: Details,
    filters: Details,
}

impl ImapFilterOperation {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ImapFilterOperation, &'static str> {
        let mut imf = ImapFilterOperation{
            lua: None,
            script_path: "".to_string(),
            im_require_path: "".to_string(),
            accounts: Details::new(),
            filters: Details::new()
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
                print!("ERR: err in LUA script: {:?}", e);
                assert!(false);
            }
        }
    }
}
