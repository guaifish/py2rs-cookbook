use std::collections::HashSet;

fn main() {
    let a = [1, 5, 2, 1, 9, 1, 5, 10];
    let mut set = HashSet::new();
    for i in a.iter() {
        set.insert(i);
    }
    println!("{:?}", set);
    let a = dedupe(&a);
    println!("{:?}", a);
}

fn dedupe(items: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut v = Vec::new();
    for item in items {
        if !seen.contains(item) {
            seen.insert(*item);
            v.push(*item);
        }
    }
    v
}