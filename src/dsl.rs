//! DSL implementation for Lua
//! All the callbacks are implemented here.
#![feature(unboxed_closures)]

use super::*;
use mlua::{Lua, Result};

macro_rules! wrap_rust_fun {
    ($lua:ident, $name:ident, $funct:ident) => {{
        let lfun = $lua.create_function($funct).unwrap();
        $lua.globals().set(stringify!($name), lfun);
        lfun
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

fn lua_account(lua: &Lua, name: String) -> Result<()> {
    println!("lua_account: {}", name);
    lambda = wrap_rust_fun!(lua, |tr|{ println!("LAMBDA: {:?}", tr); });
    Ok(lambda)
}

pub fn setup_dsl<'lua, 'callback>(lua: &Lua) -> &Lua {
    wrap_rust_fun!(lua, test_function, lua_test_function);
    wrap_rust_fun!(lua, account, lua_account);
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
