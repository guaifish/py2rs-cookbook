use linked_hash_map::LinkedHashMap;

fn main() {
    let mut m = LinkedHashMap::new();
    m.insert("foo", 1);
    m.insert("bar", 2);
    m.insert("spam", 3);
    m.insert("grok", 4);
    for (key, value) in m.iter() {
        println!("{} {}", key, value);
    }
}