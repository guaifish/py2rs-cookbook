#[macro_use] extern crate maplit;
use defaultmap::DefaultHashMap;
use std::collections::HashSet;


fn main() {
    let mut m = DefaultHashMap::new(vec![]);
    m['a'].push(1);
    m['a'].push(2);
    m['b'].push(4);
    assert_eq!(m['a'], vec![1, 2]);
    assert_eq!(m['b'], vec![4]);
    assert_eq!(m['c'], vec![]);

    let mut m = DefaultHashMap::new(HashSet::new());
    m['a'].insert(1);
    m['a'].insert(2);
    m['b'].insert(4);
    assert_eq!(m['a'], hashset!{1, 2});
    assert_eq!(m['b'], hashset!{4});
    assert_eq!(m['c'], HashSet::new());
}