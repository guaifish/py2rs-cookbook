fn main() {
    let nums = [1, 2, 3, 4, 5];
    let sum = nums.iter().map(|x| x * x).fold(0, |a, b| a + b);
    assert_eq!(sum, 55);
    let min = *nums.iter().min().unwrap();
    assert_eq!(min, 1);
    let max = *nums.iter().max().unwrap();
    assert_eq!(max, 5);
}
