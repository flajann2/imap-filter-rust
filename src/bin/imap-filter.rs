use mlua::prelude::*;

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    
    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    lua.globals().set("map_table", map_table)?;
    lua.load(r#"require "imap-filter" "#).exec()?;
    lua.load("for k,v in pairs(map_table) do print(k,v) end").exec()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lua() {
    }
}
