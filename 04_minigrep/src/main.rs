/// How to run?
///  cargo run frog poem.txt
///  cargo run body poem.txt
///  cargo run to poem.txt
///
/// Set environment variable:
///  > ls env: | ?{$_.name -like "case*"}
///  > $env:CASE_INSENSITIVE=1
///
/// Output:
///  > cargo run > output.txt
///  > cat .\output.txt
/// 
mod lib;

use lib::Config;
use std::env;
use std::process;

fn main() {
    // parse command args
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("search for: {}", config.query);
    println!("in file: {}", config.filename);

    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
