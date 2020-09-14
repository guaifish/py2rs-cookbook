#[macro_use]
extern crate maplit;

#[macro_use(c)]
extern crate cute;

use std::collections::HashMap;

fn main() {
    let prices = hashmap! {
        "ACME" => 45.23,
        "AAPL" => 612.78,
        "IBM" => 205.55,
        "HPQ" => 37.20,
        "FB" => 10.75,
    };
    
    // py
    let p1 = c! {k => v, for (k, v) in &prices, if *v > 200.0};
    println!("{:?}", p1);
    // rs
    let p1: HashMap<_, _> = prices.iter().filter(|(_, v)| **v > 200.0).collect();
    println!("{:?}", p1);

    // py
    let tech_names = hashset! {"AAPL", "IBM", "HPQ", "MSFT"};
    let p2 = c! {k => v, for (k, v) in &prices, if tech_names.contains(k)};
    println!("{:?}", p2);
    // rs
    let p2: HashMap<_, _> = prices
        .iter()
        .filter(|(k, _)| tech_names.contains(**k))
        .collect();
    println!("{:?}", p2);
}
