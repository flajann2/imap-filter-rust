//! imap library of functions

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::result::Result;
use std::fs;
use std::path::Path;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;


lrlex_mod!("imapdsl.l");
lrpar_mod!("imapdsl.y");

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
    accounts: HashMap<String, Account>,
    filters: HashMap<String, Filter>
}

impl ImapFilterOperation {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut imf = ImapFilterOperation{
            accounts: HashMap::new(),
            filters: HashMap::new()
        };
        imf.load_configuration(path)
    }

    /// TODO: Currently this will panic if the config file does not
    /// TODO: exist. Handle this properly!!!!
    fn load_configuration<P: AsRef<Path>>(mut self, path: P) -> Result<Self, String> {
        let lexerdef = imapdsl_l::lexerdef();
        let emf = fs::read_to_string(path).unwrap();

        let lexer = lexerdef.lexer(&emf);
        let (res, errs) = imapdsl_y::parse(&lexer);
        for e in errs {
            println!("{}", e.pp(&lexer, &imapdsl_y::token_epp));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test depends on a usable sample.emf to be located in the example directory.
    /// It will panic otherwise, as it should.
    ///
    /// Also, this will break on a Windows system, as they have their slashes around
    /// the wrong way. :p
    #[test]
    fn test_lexing() {
        let sample_file = Path::new(file!())
            .parent()
            .unwrap()
            .join("../example/sample.emf");
        println!("defined in file: {:?}", sample_file);
        
        let ifo = ImapFilterOperation::new(sample_file).unwrap();
    }
}
