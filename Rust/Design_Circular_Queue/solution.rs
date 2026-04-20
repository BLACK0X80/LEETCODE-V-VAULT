struct MyCircularQueue { black_q: Vec<i32>, black_h: usize, black_t: usize, black_size: usize, black_k: usize }
impl MyCircularQueue {
    fn new(k: i32) -> Self { Self { black_q: vec![-1; k as usize], black_h: 0, black_t: 0, black_size: 0, black_k: k as usize } }
    fn en_queue(&mut self, value: i32) -> bool { if self.is_full() { return false; } self.black_q[self.black_t] = value; self.black_t = (self.black_t + 1) % self.black_k; self.black_size += 1; true }
    fn de_queue(&mut self) -> bool { if self.is_empty() { return false; } self.black_h = (self.black_h + 1) % self.black_k; self.black_size -= 1; true }
    fn front(&self) -> i32 { if self.is_empty() { -1 } else { self.black_q[self.black_h] } }
    fn rear(&self) -> i32 { if self.is_empty() { -1 } else { self.black_q[(self.black_t + self.black_k - 1) % self.black_k] } }
    fn is_empty(&self) -> bool { self.black_size == 0 }
    fn is_full(&self) -> bool { self.black_size == self.black_k }
}