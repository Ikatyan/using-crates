#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref TEXT_HASHMAP: HashMap<i32, &'static str> = {
        let mut hashmap = HashMap::new();
        hashmap.insert(1, "Hello");
        hashmap.insert(2, "World");
        hashmap.insert(3, "!");
        hashmap
    };

    static ref IIYOKOIYO: i32 = 114514;
}

fn print_hello() {
    print!("{}", TEXT_HASHMAP[&1]);
}

fn print_world() {
    print!("{}", TEXT_HASHMAP[&2]);
}

fn print_exclamation() {
    println!("{}", TEXT_HASHMAP[&3]);
}

fn print_iiyokoiyo() {
    println!("{}{}", *IIYOKOIYO, TEXT_HASHMAP[&3]);
}

fn main() {
    print_hello();
    print_world();
    print_exclamation();
    print_iiyokoiyo();
}

