pub struct Stack<T> {
    pub values: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { values: vec![] }
    }
}

impl<T> Stack<T> {
    pub fn push(&mut self, val: T) {
        self.values.push(val)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn try_pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn pop(&mut self) -> T {
        self.try_pop().expect("pop of empty stack")
    }

    pub fn try_peek(&self) -> Option<&T> {
        self.values.last()
    }

    pub fn peek(&self) -> &T {
        self.try_peek().expect("peek of empty stack")
    }

    pub fn try_peek_at(&self, n: usize) -> Option<&T> {
        self.values.get(self.len() - n - 1)
    }

    pub fn peek_at(&self, n: usize) -> &T {
        self.try_peek_at(n).expect("peek of empty stack")
    }
}
