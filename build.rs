//! lua-specific build instructions

use std::path::Path;
use std::{env, fs};

const HELPER_LUA_FILE: &str = "imap-filter.lua";

fn main() {
    let target_dir_path = env::var("OUT_DIR").unwrap();

    let sfile = Path::new(&target_dir_path)
        .join("../../../../../lua")
        .join(HELPER_LUA_FILE);
    
    let tfile = Path::new(&target_dir_path)
        .join("../../..")
        .join(HELPER_LUA_FILE);

    println!("cargo:rerun-if-changed={:?}", sfile);
    
    copy(&tfile, &sfile);
}

fn copy(tfile: &Path, sfile: &Path) {
    fs::copy(sfile, &tfile).unwrap();
}
