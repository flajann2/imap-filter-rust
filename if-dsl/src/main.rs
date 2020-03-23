use std::io::{self, BufRead, Write};

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `imapdsl.l` into scope.
lrlex_mod!("imapdsl.l");

// Using `lrpar_mod!` brings the parser for `imapdsl.y` into scope.
lrpar_mod!("imapdsl.y");

fn main() {
    // Get the `LexerDef` for the `imapdsl` language.
    let lexerdef = imapdsl_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if !l.trim().is_empty() {
                    // Now we create a lexer with the `lexer` method with which
                    // we can lex an input.
                    let lexer = lexerdef.lexer(l);
                    // Pass the lexer to the parser and lex and parse the input.
                    let (res, errs) = imapdsl_y::parse(&lexer);
                    for e in errs {
                        println!("{}", e.pp(&lexer, &imapdsl_y::token_epp));
                    }
                
                    match res {
                        Some(Ok(r)) => println!("Result: {}", r),
                        _ => eprintln!("Unable to evaluate expression.")
                    }
                }
            }
            _ => break
        }
    }
}
