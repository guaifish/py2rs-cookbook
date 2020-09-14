use std::collections::BinaryHeap;

pub struct PriorityQueue<T> {
    queue: BinaryHeap<(i32, i32, T)>,
    index: i32,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            queue: BinaryHeap::<(i32, i32, T)>::new(),
            index: 0,
        }
    }

    pub fn push(&mut self, item: T, priority: i32) {
        self.queue.push((priority, -self.index, item));
        self.index += 1;
    }

    pub fn pop(&mut self) -> T {
        self.queue.pop().unwrap().2
    }
}