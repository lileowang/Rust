#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

mod lib;

use lib::log;
use std::env;
use lib::Config;
use lib::Utility;
use std::process;

fn main() {
    // println!("hello");
    // log(&"hello");

    // let args: Vec<String> = env::args().collect();
    // log(&format!("{:#?}", args));
    let config = Config::new(&mut env::args()).unwrap_or_else(|err| {
        log(&format!("{}", err));
        process::exit(1)
    });
    // log(&format!("{:#?}", config));
    log(&config.tostr1());
    log(&config.tostr2());    
}
