fn main() {
    let p = (4, 5);
    let (x, y) = p;
    assert_eq!(x, 4);
    assert_eq!(y, 5);

    let data = ("ACME", 50, 91.1, [2012, 12, 21]);
    let (name, shares, price, date) = data;
    assert_eq!(name, "ACME");
    assert_eq!(shares, 50);
    assert_eq!(price, 91.1);
    assert_eq!(date, [2012, 12, 21]);

    let s = "Hello";
    let s: Vec<_> = s.chars().collect();
    if let [a, b, c, d, e] = s[..] {
        assert_eq!(a, 'H');
        assert_eq!(b, 'e');
        assert_eq!(c, 'l');
        assert_eq!(d, 'l');
        assert_eq!(e, 'o');
    };

    let data = ("ACME", 50, 91.1, [2012, 12, 21]);
    let (_, shares, price, _) = data;
    assert_eq!(shares, 50);
    assert_eq!(price, 91.1);
}
