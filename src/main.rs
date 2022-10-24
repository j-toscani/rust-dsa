mod list_queue;
mod node_queue;

fn main() {
    let mut q = list_queue::Queue::new();
    q.queue(16);
    q.queue(32);

    let first = q.deque().unwrap();
    let second = q.deque().unwrap();
    println!("Elements in Order: {}, {}", first, second);

    let mut nq = node_queue::NodeQueue::new();
    nq.queue(33);
    nq.queue(4);
    println!("The value is: {}", nq.peek().unwrap());

    let number = nq.deque().unwrap();
    println!("Dequeued {}", &number);

    println!("New head is {:?}", nq.peek())
}
