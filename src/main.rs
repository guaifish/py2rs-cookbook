use std::collections::BinaryHeap;

trait Largest<T> {
    fn nlargest(&self, n: u32) -> Vec<T>;
}

impl<T: Ord + Copy> Largest<T> for BinaryHeap<T> {
    fn nlargest(&self, n: u32) -> Vec<T> {
        let mut heaq = self.clone();
        let mut v: Vec<T> = Vec::new();
        if n > self.len() as u32 {
            panic!("n is bigger than heaq's lengh")
        }
        for _ in 0..n {
            if let Some(x) = heaq.pop() {
                v.push(x)
            } else {
                unimplemented!()
            }
        }
        v
    }
}

fn main() {
    let nums = vec![1, 8, 2, 23, 7, -4, 18, 23, 42, 37, 2];
    // 最大堆
    let heap = BinaryHeap::from(nums);
    println!("{:?}", heap.nlargest(3));
}
