use num_rational::Ratio;

fn main() {
    let a = Ratio::new(5, 4);
    let b = Ratio::new(7, 16);
    let c = Ratio::new(35, 64);
    assert_eq!(a + b, Ratio::new(27, 16));
    assert_eq!(a * b, c);
    assert_eq!(c.numer(), &35);
    assert_eq!(c.denom(), &64);
}
