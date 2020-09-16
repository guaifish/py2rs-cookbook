use ndarray::array;

fn main() {
    let a1 = array![1, 2, 3, 4];
    let a2 = array![2, 4, 6, 8];
    let a3 = array![11, 12, 13, 14];
    let a4 = array![5, 6, 7, 8];
    let a5 = array![5, 12, 21, 32];
    assert_eq!(&a1 * 2, a2);
    assert_eq!(&a1 + 10, a3);
    assert_eq!(&a1 * &a4, a5);

    let a = array![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
}
