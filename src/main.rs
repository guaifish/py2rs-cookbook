fn main() {
    assert_eq!(round(1.23, 1), 1.2);
    assert_eq!(round(1.27, 1), 1.3);
    assert_eq!(round(1.25361, 3), 1.254);
    assert_eq!(round(1627731.0, -1), 1627730.0);
    assert_eq!(round(1627731.0, -2), 1627700.0);
    assert_eq!(round(1627731.0, -3), 1628000.0);
}

fn round(value: f64, ndigits: i32) -> f64 {
    let x = 10.0_f64.powi(ndigits);
    (value * x).round() / x
}
