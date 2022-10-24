pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct NodeQueue<T> {
    head: Option<Node<T>>,
    length: u32,
}

impl<T> NodeQueue<T> {
    pub fn new() -> NodeQueue<T> {
        return NodeQueue {
            length: 0,
            head: None,
        };
    }
    pub fn queue(&mut self, item: T) -> () {
        let node = Node {
            value: item,
            next: None,
        };
        self.length = self.length + 1;

        match self.head {
            None => {
                self.head = Some(node);
            }
            Some(ref mut head) => {
                let mut tail = head;
                loop {
                    match tail.next {
                        Some(ref mut next_node) => {
                            tail = next_node;
                        }
                        None => {
                            break;
                        }
                    }
                }
                tail.next = Some(Box::new(node));
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        match self.head {
            Some(_) => false,
            None => true,
        }
    }
    pub fn deque(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            self.length = self.length - 1;
            let head = self.head.take().unwrap();

            if let Some(next) = head.next {
                self.head = Some(*next);
            }
            Some(head.value)
        }
    }
    pub fn peek(&self) -> Option<&T> {
        return match &self.head {
            Some(node) => Some(&node.value),
            None => None,
        };
    }
}
