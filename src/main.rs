fn main() {
    let filename = "spam.txt";
    assert_eq!(filename.ends_with(".txt"), true);

    let url = "http://www.python.org";
    assert_eq!(url.starts_with("http:"), true);
}
