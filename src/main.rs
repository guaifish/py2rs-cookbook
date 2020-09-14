#[macro_use]
extern crate maplit;

fn main() {
    let mut rows = [
        hashmap! {"fname" => "Brian", "lname" => "Jones", "uid" => "1003"},
        hashmap! {"fname" => "David", "lname" => "Beazley", "uid" => "1002"},
        hashmap! {"fname" => "John", "lname" => "Cleese", "uid" => "1001"},
        hashmap! {"fname" => "Big", "lname" => "Jones", "uid" => "1004"},
    ];
    rows.sort_by(|a, b| {
        let a: i32 = a["uid"].parse().unwrap();
        let b: i32 = b["uid"].parse().unwrap();
        a.cmp(&b)
    });
    for r in rows.iter() {
        println!("{:?}", r);
    }
}
