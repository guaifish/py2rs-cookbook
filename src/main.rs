#[macro_use]
extern crate decimal;

fn main() {
    let a = 4.2;
    let b = 2.1;
    assert!(equal(a + b, 6.3));

    let a = d128!(4.2);
    let b = d128!(2.1);
    assert_eq!(a + b, d128!(6.3));
}

fn equal(a: f64, b: f64) -> bool {
    a - b < 0.1_f64.powi(10)
}
