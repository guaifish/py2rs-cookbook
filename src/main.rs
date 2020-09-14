#[macro_use]
extern crate maplit;
use chain_map::ChainMap;

fn main() {
    let a = hashmap! { 'x' => 1, 'z' => 3 };
    let b = hashmap! { 'y' => 2, 'z' => 4 };
    let mergegd: ChainMap<_, _> = vec![a, b].into_iter().collect();
    assert_eq!(mergegd[&'x'], 1);
    assert_eq!(mergegd[&'y'], 2);
    assert_eq!(mergegd[&'z'], 3);
}
