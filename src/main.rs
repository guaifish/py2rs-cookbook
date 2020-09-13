use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::with_capacity(3);
    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    q.push_front(4);
    assert_eq!(q, VecDeque::from(vec![4, 1, 2, 3])); // 没有定长
}
