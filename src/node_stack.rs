pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

pub struct Stack<T> {
    next: Option<Node<T>>,
    length: i32
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        return Node {
            value,
            next: None
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        return Stack {
            length: 0,
            next: None
        }
    }

    pub fn push(&mut self, value: T) {
        let mut node = Node::new(value);
        self.length = self.length + 1;

        if self.next.is_some() {
            let next_node = self.next.take().unwrap();
            node.next = Some(Box::new(next_node));
            self.next = Some(node);
        } else {
            self.next = Some(node);   
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let next = self.next.take();

        match next {
            Some(next_node) => {
                let value = next_node.value;
                self.next = match next_node.next {
                    Some(new_next) => Some(*new_next),
                    None => None
                };

                Some(value)
            },
            None => None
        }
    }   
}

