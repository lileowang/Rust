#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

mod lib;

use lib::log;
use lib::Config;
use std::process;

fn main() {
    // log(&"hello");

    let config = Config {
        max_attempts: 10,
        max_range: 100,
    };
    // log(&format!("{:#?}", config));
    if let Err(e) = lib::run(&config) {
        log(&e);
        process::exit(2)
    };
}
