//! As the name suggests, conversion between Lua and Rust
//! data structures.

// TODO: remove the following beofre release
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]

use super::*;
use mlua::{Lua, Result, Function, Table, Value};

pub fn toRust(ltable: &Table) -> Details {
    let mut det = Details::new();
    det
}

