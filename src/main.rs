fn main() {
    let items = [0, 1, 2, 3, 4, 5, 6];
    let a = 2..4;
    assert_eq!(a.start, 2);
    assert_eq!(a.end, 4);
    println!("{:?}", &items[a]);
}
