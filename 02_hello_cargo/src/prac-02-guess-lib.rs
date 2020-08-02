#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use rand::Rng;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::{io, cmp::Ordering};

#[derive(Debug)]
pub struct Config {
    pub max_attempts: u8,
    pub max_range: u32,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // return Err("unknown failure")?;
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
                continue;
            }
        }
    }

    log(&format!("secret number: {}", secret_num));
    Ok(())
}

fn check_guess(guess: &str, secret_num: u32) -> Result<&'static str, &'static str> {
    let guess = match guess.parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err("not a number")
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

        assert_eq!("not a number", ret);
    }

    #[test]
    fn check_guess_too_small() {
        let guess = "9";
        let secret_num = 10;
        let ret = match check_guess(&guess, secret_num) {
            Ok(n) => n,
            Err(e) => e,
        };

        assert_eq!("too small", ret);
    }

    #[test]
    fn check_guess_too_big() {
        let guess = "11";
        let secret_num = 10;
        let ret = match check_guess(&guess, secret_num) {
            Ok(n) => n,
            Err(e) => e,
        };

        assert_eq!("too big", ret);
    }

    #[test]
    fn check_guess_you_win() {
        let guess = "10";
        let secret_num = 10;
        let ret = match check_guess(&guess, secret_num) {
            Ok(n) => n,
            Err(e) => e,
        };

        assert_eq!("you win", ret);
    }
}

pub fn log<T>(msg: &T)
where
    T: Display + Debug,
{
    println!("{}", msg);
}
