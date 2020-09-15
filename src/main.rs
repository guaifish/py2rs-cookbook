fn main() {
    let s = " hello world \n";
    assert_eq!(s.trim(), "hello world");
    assert_eq!(s.trim_start(), "hello world \n");
    assert_eq!(s.trim_end(), " hello world");

    let t = "-----hello=====";
    assert_eq!(t.trim_start_matches('-'), "hello=====");
    assert_eq!(t.trim_end_matches('='), "-----hello");
    let pat: &[_] = &['-', '='];
    assert_eq!(t.trim_matches(pat), "hello");
}
