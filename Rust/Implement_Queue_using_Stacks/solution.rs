struct MyQueue {
    black_in: Vec<i32>,
    black_out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue { black_in: vec![], black_out: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.black_in.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.black_out.is_empty() {
            while let Some(v) = self.black_in.pop() { self.black_out.push(v); }
        }
        self.black_out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.black_out.is_empty() {
            while let Some(v) = self.black_in.pop() { self.black_out.push(v); }
        }
        *self.black_out.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.black_in.is_empty() && self.black_out.is_empty()
    }
}