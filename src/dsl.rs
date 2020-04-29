//! DSL implementation for Lua
//! All the callbacks are implemented here.
#![feature(unboxed_closures)]

use super::*;
use mlua::{Lua, Result, Function, Table, Value};
use std::vec::*;

macro_rules! wrap_rust_fun {
    ($lua:ident, $name:ident, $funct:ident) => {{
        let lfun = $lua.create_function($funct).unwrap();
        $lua.globals().set(stringify!($name), lfun);
    }}
}

macro_rules! wrap_rust_lambda {
    ($lua:ident, $lambda:expr) => {
        $lua.create_function($lambda).unwrap()
    }
}

fn lua_test_function(lua: &Lua, s: String) -> Result<String> {
    println!("LTEST: lua_test_function() called with {}", s);
    Ok(s)
}

fn lua_account(lua: &Lua, name: String) -> Result<Function> {
    println!("lua_account: {}", name);
    let lambda = wrap_rust_lambda!(lua, |_, table: Table| {
        println!("LAMBDA_lua_account:");
        for pair in table.pairs::<Value, Value>() {
            let (key, value) = pair?;
            println!("    {:?} => {:?}", key, value);
        }
        Ok(())
    });
    Ok(lambda)
}

fn lua_filter(lua: &Lua, name: String) -> Result<Function> {
    println!("lua_filter: {}", name);
    let lambda = wrap_rust_lambda!(lua, |_, table: Table| {
        println!("LAMBDA_lua_filter:");
        for pair in table.pairs::<Value, Value>() {
            let (key, value) = pair?;
            println!("    {:?} => {:?}", key, value);
        }
        Ok(())
    });
    Ok(lambda)
}

pub fn setup_dsl<'lua, 'callback>(lua: &Lua) -> &Lua {
    wrap_rust_fun!(lua, test_function, lua_test_function);
    wrap_rust_fun!(lua, account, lua_account);
    wrap_rust_fun!(lua, filter, lua_filter);
    &lua
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_dsl() {
        match ImapFilterOperation::new(SIMPLE) {
            Ok(mut ifo) => {
                let lua = ifo._lua();
                match lua.load(r#"test_function("Hello World to Rust")"#)
                    .exec() {
                        Ok(r) => println!("RTEST: {:?}", r),
                        Err(e) => {
                            println!("RERR: {:?}", e);
                            assert!(false);
                        },
                    }
                ifo.run();
            },
            Err(e) => {
                print!("err in LUA script: {:?}", e);
                assert!(false);
            }
        }
    }
}
