#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::borrow::Cow;

fn is_date(text: &str) -> bool {
    let regex = Regex::new(r"\A\d{4}-\d{2}-\d{2}\z").unwrap();
    regex.is_match(text)
}

fn to_mmddyyyy(text: &str) -> Cow<str> {
    let regex = Regex::new(r"\A(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\z").unwrap();
    regex.replace_all(text, "$month$day$year")
}

fn anti_pattern(text: &str) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();
    let regex = Regex::new(r"\A\d{4}-\d{2}-\d{2}\z").unwrap();
    for _ in 0..1000 {
        vec.push(regex.is_match(text));
    }
    vec
}

fn best_practice(text: &str) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();
    lazy_static! {
        static ref REGEX: Regex = {
            Regex::new(r"\A(\d{4})-(\d{2})-(\d{2})\z").unwrap()
        };
    }
    for _ in 0..1000 {
        vec.push(REGEX.is_match(text));
    }
    vec
}

fn main() {
    let text = "1992-07-18";
    println!("{}", is_date(text));
    println!("{}", to_mmddyyyy(text));
    println!("anti_pattern:{:?}", anti_pattern(text));
    println!("best_practice:{:?}", best_practice(text));
}
