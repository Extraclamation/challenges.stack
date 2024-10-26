/// A struct implementing a basic LIFO stack.
/// ```
/// let mut stack: stack::Stack<u8> = stack::Stack::new();
/// stack.push(1);
/// stack.push(2);
/// println!("{:?}, {:?}", stack.pop(), stack.pop());  // 2, 1
/// ```
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<T> Stack<T> {
    /// Create a new empty stack.
    /// ```
    /// let mut stack: stack::Stack<u8> = stack::Stack::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Pushes an item to the end of stack.
    /// ```
    /// # let mut stack: stack::Stack<u8> = stack::Stack::new();
    /// println!("{:?}", stack.push(1)); // 1
    /// println!("{:?}", stack.push(2)); // 1, 2
    /// ```
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    /// Pops an item from the end of the stack. Returns None if the stack is empty.
    /// ```
    /// # let mut stack: stack::Stack<u8> = stack::Stack::new();
    /// stack.push(1);
    /// println!("{:?}", stack.pop()); // Some(1)
    /// println!("{:?}", stack.pop()); // None
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Returns the item at the end of the stack without removing it. Returns None if the stack is empty.
    /// ```
    /// # let mut stack: stack::Stack<u8> = stack::Stack::new();
    /// stack.push(1);
    /// println!("{:?}", stack.peek()); // Some(1)
    /// println!("{:?}", stack.pop()); // Some(1)
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    /// Returns true if the stack is empty.
    /// ```
    /// # let mut stack: stack::Stack<u8> = stack::Stack::new();
    /// stack.push(1);
    /// println!("{:?}", stack.is_empty()); // false
    /// stack.pop();
    /// println!("{:?}", stack.is_empty()); // true   
    /// ```
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
