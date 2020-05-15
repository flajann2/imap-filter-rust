//! DSL implementation for Lua
//! All the callbacks are implemented here.
#![feature(unboxed_closures)]

use super::*;
use mlua::{Lua, Result, Function, Table, Value, Error::*};
use std::{vec::*, env};

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
    let lambda = wrap_rust_lambda!(lua, |ll: &Lua, table: Table| {
        println!("LAMBDA_lua_account:");
        for pair in table.pairs::<Value, Value>() {
            let (key, value) = pair?;
            println!("    {:?} => {:?}", key, value);
        }
        Ok(())
    });
    Ok(lambda)
}

fn lua_login(lua: &Lua, table: Table) -> Result<String> {
    println!("lua_login:");
    for pair in table.pairs::<Value, Value>() {
        let (key, value) = pair?;
        println!("    {:?} => {:?}", key.unwrap(), value);
    }
    Ok("whaaa".to_string())
}

fn lua_env(lua: &Lua, key: String) -> Result<String> {
    println!("lua_env: {}", key);
    match env::var_os(key) {
        Some(v) => Ok(v.into_string().unwrap()),
        None => Err(RuntimeError("key not found".to_string()))
    }
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

pub fn setup_dsl<'lua, 'callback>(ifo: &ImapFilterOperation, lua: &'lua Lua) -> &'lua Lua {
    wrap_rust_fun!(lua, test_function, lua_test_function);

    wrap_rust_fun!(lua, account, lua_account);
    wrap_rust_fun!(lua, login, lua_login);
    wrap_rust_fun!(lua, env, lua_env);

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
