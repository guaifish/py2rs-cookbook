fn main() {
    let x = 1234.56789;
    assert_eq!(format!("{:.2}", x), "1234.57");
    assert_eq!(format!("{:>10.1}", x), "    1234.6");
    assert_eq!(format!("{:<10.1}", x), "1234.6    ");
    assert_eq!(format!("{:^10.1}", x), "  1234.6  ");
    assert_eq!(format!("{:.1e}", x), "1.2e3");
    assert_eq!(format!("{:.1E}", x), "1.2E3");
}
