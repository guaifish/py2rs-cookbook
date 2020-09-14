#[macro_use]
extern crate maplit;

fn main() {
    let prices = hashmap! {
        "ACME" => 45.23,
        "AAPL" => 612.78,
        "IBM" => 205.55,
        "HPQ" => 37.20,
        "FB" => 10.75,
    };
    // note: 浮点数数组最大值, f64 没有实现 Ord
    let max_prices_and_names = prices
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    println!("{:?}", max_prices_and_names);

    let min_prices_and_names = prices
        .iter()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    println!("{:?}", min_prices_and_names);
}
