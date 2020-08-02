// Author:          Li Leo Wang
// Date started:    2020-06-20
// Description:
//  - lib.rs as logic implementation
//

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;
use std::fmt::{Debug, Display};
use std::{cmp::Ordering, io};

pub fn run(config: &Config) {
    let secret_num = rand::thread_rng().gen_range(1, config.max_range + 1);

    for i in (1..config.max_attempts + 1).rev() {
        log(&format!("remaining: {}", i));

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");
        guess = guess.trim().to_string();
        if guess.eq_ignore_ascii_case("q") {
            break;
        }

        match check_guess(&guess, secret_num) {
            Ok(n) => {
                log(&n);
                break;
            }
            Err(e) => {
                log(&e);
            }
        }
    }

    log(&format!("secret number: {}", secret_num));
}

fn check_guess(guess: &str, secret_num: i32) -> Result<&'static str, &'static str> {
    let guess: i32 = match guess.parse() {
        Ok(n) => n,
        Err(_) => {
            return Err("not a number");
        }
    };

    match guess.cmp(&secret_num) {
        Ordering::Less => Err("too small"),
        Ordering::Greater => Err("too big"),
        Ordering::Equal => Ok("you win"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_guess_not_a_number() {
        let guess = "a";
        let secret_num = 10;
        let ret = match check_guess(&guess, secret_num) {
            Ok(n) => n,
            Err(e) => e,
        };
        assert_eq!("not a number".to_lowercase(), ret.to_lowercase());
    }

    #[test]
    fn check_guess_too_small() {
        let guess = "9";
        let secret_num = 10;
        let ret = match check_guess(&guess, secret_num) {
            Ok(n) => n,
            Err(e) => e,
        };
        assert_eq!("too small".to_lowercase(), ret.to_lowercase());
    }

    #[test]
    fn check_guess_not_big() {
        let guess = "11";
        let secret_num = 10;
        let ret = match check_guess(&guess, secret_num) {
            Ok(n) => n,
            Err(e) => e,
        };
        assert_eq!("too big".to_lowercase(), ret.to_lowercase());
    }

    #[test]
    fn check_guess_you_win() {
        let guess = "10";
        let secret_num = 10;
        let ret = match check_guess(&guess, secret_num) {
            Ok(n) => n,
            Err(e) => e,
        };
        assert_eq!("you win".to_lowercase(), ret.to_lowercase());
    }
}

pub trait Utility {
    fn tostr1(&self) -> String;
}

pub struct Config {
    pub max_attempts: u8,
    pub max_range: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("need 2 args");
        }

        let max_attempts = match args[1].parse::<u8>() {
            Ok(n) => n,
            Err(_) => return Err("max_attempts is not u8"),
        };

        let max_range = match args[2].parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err("max_range is not i32"),
        };
        Ok(Config {
            max_attempts,
            max_range,
        })
    }
}

impl Config {
    pub fn tostr2(&self) -> String {
        format!(
            "max_attempts: {}, max_range: {}",
            self.max_attempts, self.max_range
        )
    }
}

impl Utility for Config {
    fn tostr1(&self) -> String {
        format!(
            "max_attempts = {}, max_range = {}",
            self.max_attempts, self.max_range
        )
    }
}

pub fn log<T>(msg: &T)
where
    T: Display + Debug,
{
    println!("{}", msg);
}
