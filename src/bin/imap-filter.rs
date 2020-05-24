//! IMAP Filter command-line application.

// TODO: remove the following beofre release
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

#[macro_use]
extern crate clap;

extern crate clap_nested;
extern crate imap_filter;

use mlua::prelude::*;
use clap::Arg;
use clap_nested::Commander;
use imap_filter::*;
use imap_filter::command::{check, run};

fn main() -> Result<(), &'static str> {
    let matches = clap_app!(myapp =>
     (version: "0.0.0")
     (author: "Fred Mithell <fred.mitchell@gmx.de>")
     (about: "IMAP Filter -- a client-independent way to filter your email across many accounts.")
     (@arg CONFIG: -c --config +takes_value "config file?")
     (@arg filter: +required "Specifies the Lua filter to use")
     (@arg debug: -d "Turns on the debugger.")
     (@arg trial: -t --trial "Trial (dry) run. Does not make any changes. It will log into all IMAP accounts and fail should a login fails.")
    ).get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("~/.imap_filter.conf");
    let filter = matches.value_of("filter").unwrap();
    let debug = matches.value_of("debug");
    let trial = matches.value_of("trial");
    
    println!("Value for config: {}, and for filter {}", config, filter);
    match ImapFilterOperation::init(filter) {
        Ok(()) => {
            IFO.with( |ifo| {
                let b = &*ifo.borrow();
                b.as_ref().unwrap().run();
            });
            Ok(())
        },
        Err(s) => Err(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lua_config_sample() {
    }
}
