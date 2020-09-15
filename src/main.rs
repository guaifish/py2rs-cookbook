fn main() {
    let data = b"Hello World";
    assert_eq!(&data[0..5], b"Hello");
    assert_eq!(String::from_utf8_lossy(&data[0..5]), "Hello");
    let mut iter = data.split(|x| *x == b' ');
    assert_eq!(iter.next().unwrap(), b"Hello");
    assert_eq!(iter.next().unwrap(), b"World");
}
