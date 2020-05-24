//! As the name suggests, conversion between Lua tables and Rust
//! data structures. We do this recursively, of course.

// TODO: remove the following beofre release
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]

use super::*;
use mlua::{Lua, Result, Function, Table, Value};

pub fn to_rust(ltable: &Table) -> Details {
    let mut det = Details::new();
    det
}
