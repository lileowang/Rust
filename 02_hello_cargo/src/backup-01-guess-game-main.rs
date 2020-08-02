// Author:          Li Leo Wang
// Date started:    2020-06-20
// Description:
//  - main.rs as program entry point
//  - Guessing game based on Rust "the book"
//  - Added all possible tricks learned from the book.
//  - It is used for daily practice.
//
// How to use?
//   cargo run 10 100
//

#![allow(unused_variables)]
#![allow(unused_imports)]

mod lib;

use crate::lib::Config;
use crate::lib::Utility;
use std::env;
use std::process;

fn main() {
    // println!("hello");
    // lib::log(&"hello");

    let args: Vec<String> = env::args().collect();
    // lib::log(&format!("{:#?}", &args));
    let config = Config::new(&args).unwrap_or_else(|err| {
        lib::log(&format!("wrong args: {}", err));
        process::exit(1)
    });

    // lib::log(&config.tostr1());
    // lib::log(&config.tostr2());

    lib::run(&config);
}
