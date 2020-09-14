use py2rs_cookbook::PriorityQueue;

fn main() {
    let mut q = PriorityQueue::new();
    q.push("foo", 1);
    q.push("bar", 5);
    q.push("spam", 4);
    q.push("grok", 1);
    assert_eq!(q.pop(), "bar");
    assert_eq!(q.pop(), "spam");
    assert_eq!(q.pop(), "foo");
    assert_eq!(q.pop(), "grok");
}
