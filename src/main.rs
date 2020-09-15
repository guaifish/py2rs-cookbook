fn main() {
    assert_ne!(0.0_f64 / 0.0_f64, 0.0_f64 / 0.0_f64);
    assert_ne!(f64::NAN, f64::NAN);
    assert_eq!(1.0_f64 / 0.0_f64, f64::INFINITY);
    assert_eq!(-1.0_f64 / 0.0_f64, f64::NEG_INFINITY);

    assert_eq!(f64::INFINITY + 45.0, f64::INFINITY);
    assert_eq!(f64::INFINITY * 10.0, f64::INFINITY);
    assert_eq!(10.0 / f64::INFINITY, 0.0);

    assert_eq!(format!("{:?}", f64::INFINITY / f64::INFINITY), "NaN");
    assert_eq!(format!("{:?}", f64::INFINITY / f64::NEG_INFINITY), "NaN");
    assert_eq!(format!("{:?}", f64::NAN + 23.0), "NaN");
    assert_eq!(format!("{:?}", f64::NAN / 2.0), "NaN");
    assert_eq!(format!("{:?}", f64::NAN * 2.0), "NaN");
    assert_eq!(format!("{:?}", f64::NAN.sqrt()), "NaN");
}
