#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Data {
    hello: i32,
    world: i32,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
struct Data2 {
    data: Rc<Data>,
    amazon: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
struct Data3 {
    data: Rc<Data2>,
    amazon: Vec<i32>,
}

fn main() {
    let data1 = Rc::new(Data { hello: 32, world: 42 });
    let data2 = Rc::new(Data2 { data: data1.clone(), amazon: "amazon".to_string() });
    let data3 = Data3 { data: data2.clone(), amazon: vec![12, 13, 14, 15] };
    let serialized = serde_json::to_string(&data1).unwrap();
    let serialized2 = serde_json::to_string(&data2).unwrap();
    let serialized3 = serde_json::to_string(&data3).unwrap();
    println!("{}", serialized);
    println!("{}", serialized2);
    println!("{}", serialized3);
}