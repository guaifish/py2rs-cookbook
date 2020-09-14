#[macro_use]
extern crate maplit;

fn main() {
    let a = hashset! {1, 2, 3};
    let b = hashset! {4, 2, 3};

    println!("{:?}", a.union(&b));
    println!("{:?}", a.difference(&b));
    println!("{:?}", a.intersection(&b));

    let a = hashmap! {
        'x' => 1,
        'y' => 2,
        'z' => 3,
    };
    let b = hashmap! {
        'w' => 10,
        'x' => 11,
        'y' => 2,
    };
    // HashMap 无法交并差集合操作
    println!("{:?}", a.union(&b));
    println!("{:?}", a.difference(&b));
    println!("{:?}", a.intersection(&b));
}
