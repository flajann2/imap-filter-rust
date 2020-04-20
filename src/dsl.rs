//! DSL implementation for Lua
//! All the callbacks are implemented here.
use mlua::{Lua, Result};

fn lua_account(lua: &Lua, name: String) -> Result<()> {
    Ok(())
}

pub fn setup_dsl<'lua, 'callback>(lua: &Lua) -> &Lua {
    let f = lua.create_function(lua_account);
    &lua
}
