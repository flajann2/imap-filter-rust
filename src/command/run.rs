//! Run the lua script for imap filter

use clap::Arg;
use clap_nested::Command;

pub fn get_cmd<'a>() -> Command<'a, str> {
    Command::new("run")
        .description("Run the filter")
        .options( |app| {
            app.arg(
                Arg::with_name("debug")
                    .short("d")
                    .help("Prints debug information verbosely"),
            )
        })
        .runner( |args, matches| {
            let debug = clap::value_t!(matches, "debug", bool).unwrap_or_default();
            println!("Running foo, env = {}, debug = {}", args, debug);
            Ok(())
        })
}

