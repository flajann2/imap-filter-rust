//! Command line parsing
//! TODO: Deprecated. Delete this.

extern crate clap;
use clap::Arg;
use clap_nested::Command;

pub fn get_cmd<'a>() -> Command<'a, str> {
    Command::new("check")
        .description("Check that all accounts connections are valid. ")
        .options(|app| {
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
