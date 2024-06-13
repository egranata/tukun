use crate::{runtime_module::RuntimeCallable, stack::Stack};

#[derive(Default)]
pub struct Unwinder {
    b: Stack<String>,
}

impl Unwinder {
    fn do_push_frame(&mut self, s: &str) {
        self.b.push(s.to_owned())
    }
    pub fn push_frame(&mut self, frame: &RuntimeCallable) {
        self.do_push_frame(&frame.fullname())
    }

    pub fn pop_frame(&mut self) -> String {
        self.b.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.b.is_empty()
    }

    pub fn len(&self) -> usize {
        self.b.len()
    }
}

impl std::fmt::Display for Unwinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::default();
        let mut first = true;
        let mut i = 0;
        while i < self.len() {
            let frame = self.b.peek_at(i);
            if first {
                first = false;
                s = frame.to_owned();
            } else {
                s = s + "\n" + frame;
            }
            i += 1;
        }

        write!(f, "{}", s)
    }
}
