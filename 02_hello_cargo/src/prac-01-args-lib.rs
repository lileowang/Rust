#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use std::error::Error;
use std::fmt::{Debug, Display};

pub fn log<T>(msg: &T)
where
    T: Display + Debug,
{
    println!("{}", msg);
}

pub trait Utility {
    fn tostr1(&self) -> String;
}

#[derive(Debug)]
pub struct Config {
    max_attempts: u8,
    max_range: u32,
}

impl Config {
    pub fn new(args: &mut std::env::Args) -> Result<Self, Box<dyn Error>> {
        args.next();

        let max_attempts = match args.next() {
            Some(n) => n,
            None => return Err("max_attempts is missing")?,
        };
        let max_range = match args.next() {
            Some(n) => n,
            None => return Err("max_range is missing")?,
        };

        let max_attempts = match max_attempts.parse::<u8>() {
            Ok(n) => n,
            Err(_) => return Err("max_attempts is not u8")?,
        };
        let max_range = match max_range.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err("max_range is not u32")?,
        };

        Ok(Config {
            max_attempts,
            max_range,
        })
    }
}

impl Utility for Config {
    fn tostr1(&self) -> String {
        format!(
            "max_attempts: {}, max_range: {}",
            self.max_attempts, self.max_range,
        )
    }
}

impl Config {
    pub fn tostr2(&self) -> String {
        format!(
            "max_attempts = {}, max_range = {}",
            self.max_attempts, self.max_range,
        )
    }
}
