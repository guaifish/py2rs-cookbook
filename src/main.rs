fn main() {
    let x = 1234;
    let bin = "10011010010";
    let oct = "2322";
    let hex = "4d2";
    assert_eq!(format!("{:b}", x), bin);
    assert_eq!(format!("{:#b}", x), "0b".to_owned() + bin);
    assert_eq!(format!("{:o}", x), oct);
    assert_eq!(format!("{:#o}", x), "0o".to_owned() + oct);
    assert_eq!(format!("{:x}", x), hex);
    assert_eq!(format!("{:#x}", x), "0x".to_owned() + hex);

    assert_eq!(i32::from_str_radix(bin, 2).unwrap(), x);
    assert_eq!(i32::from_str_radix(oct, 8).unwrap(), x);
    assert_eq!(i32::from_str_radix(hex, 16).unwrap(), x);
}
