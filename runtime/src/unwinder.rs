use crate::{frame::Frame, runtime_module::RuntimeCallable, stack::Stack};

#[derive(Default)]
pub struct Unwinder {
    b: Stack<Frame>,
}

impl Unwinder {
    pub fn push_frame(&mut self, f: &RuntimeCallable) -> &Frame {
        let frm = Frame::new(f);
        self.b.push(frm);
        self.b.peek()
    }

    pub fn set_ip(&mut self, ip: usize) -> &Frame {
        self.b.peek_mut().set_ip(ip);
        self.b.peek()
    }

    pub fn pop_frame(&mut self) -> Frame {
        self.b.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.b.is_empty()
    }

    pub fn len(&self) -> usize {
        self.b.len()
    }

    pub fn unwind(&self) -> Stack<Frame> {
        self.b.clone()
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
                s = format!("{}", frame);
            } else {
                s = format!("{}\n{}", s, frame);
            }
            i += 1;
        }

        write!(f, "{}", s)
    }
}
