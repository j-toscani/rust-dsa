pub struct Queue {
    items: Vec<i32>,
}

impl Queue {
    pub fn new() -> Self {
        return Queue { items: Vec::new() };
    }
    pub fn queue(&mut self, item: i32) {
        self.items.push(item);
    }
    pub fn deque(&mut self) -> Option<i32> {
        if self.items.len() > 0 {
            Some(self.items.remove(0))
        } else {
            None
        }
    }
    pub fn peek(&self) -> &i32 {
        &self.items[0]
    }
}
