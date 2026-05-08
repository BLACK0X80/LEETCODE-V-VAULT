struct MyCircularDeque { black_v: Vec<i32>, black_h: usize, black_t: usize, black_c: usize, black_k: usize }
impl MyCircularDeque {
    fn new(k: i32) -> Self { Self { black_v: vec![0; k as usize], black_h: 0, black_t: 0, black_c: 0, black_k: k as usize } }
    fn insert_front(&mut self, value: i32) -> bool { if self.is_full() { false } else { self.black_h = (self.black_h + self.black_k - 1) % self.black_k; self.black_v[self.black_h] = value; self.black_c += 1; if self.black_c == 1 { self.black_t = (self.black_h + 1) % self.black_k; } true } }
    fn insert_last(&mut self, value: i32) -> bool { if self.is_full() { false } else { self.black_v[self.black_t] = value; self.black_t = (self.black_t + 1) % self.black_k; self.black_c += 1; true } }
    fn delete_front(&mut self) -> bool { if self.is_empty() { false } else { self.black_h = (self.black_h + 1) % self.black_k; self.black_c -= 1; true } }
    fn delete_last(&mut self) -> bool { if self.is_empty() { false } else { self.black_t = (self.black_t + self.black_k - 1) % self.black_k; self.black_c -= 1; true } }
    fn get_front(&self) -> i32 { if self.is_empty() { -1 } else { self.black_v[self.black_h] } }
    fn get_rear(&self) -> i32 { if self.is_empty() { -1 } else { self.black_v[(self.black_t + self.black_k - 1) % self.black_k] } }
    fn is_empty(&self) -> bool { self.black_c == 0 }
    fn is_full(&self) -> bool { self.black_c == self.black_k }
}