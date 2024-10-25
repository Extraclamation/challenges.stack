pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.first()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[test]
fn test_pop_order() {
    let mut stack: Stack<u8> = Stack::new();
    stack.push(1);
    stack.push(4);

    assert!(stack.pop() == Some(4));
    assert!(stack.pop() == Some(1));
    assert!(stack.pop().is_none());
}
