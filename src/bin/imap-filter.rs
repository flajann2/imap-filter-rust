//! IMAP Filter command-line application.

extern crate clap_nested;

use mlua::prelude::*;
use clap::Arg;
use clap_nested::Commander;

use crate::command::check;
use crate::command::run;

fn main() {
    Commander::new()
        .options(|app| {
            app.arg(
                Arg::with_name("environment")
                    .short("e")
                    .long("env")
                    .global(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Sets an environment value, defaults to \"dev\""),
            )
        })
        .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
        .add_cmd(check::get_cmd())
        .add_cmd(run::get_cmd())
        .no_cmd( |_args, _matches| {
            println!("No subcommand matched");
            Ok(())
        })
        .run()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lua_config_sample() {
    }
}
