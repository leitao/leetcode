use priority_queue::PriorityQueue;

fn main() {
    let mut pq = PriorityQueue::new();
    pq.push("foo", 1);
    pq.push("baz", 15);
    pq.push("bar", 12);
    println!("{:?}", pq);
    println!("{:?}", pq.peek());
}
