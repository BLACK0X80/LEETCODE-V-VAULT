struct CustomStack { black_stack: Vec<i32>, black_inc: Vec<i32>, black_size: usize }
impl CustomStack {
    fn new(max_size: i32) -> Self { Self { black_stack: Vec::with_capacity(max_size as usize), black_inc: vec![0; max_size as usize], black_size: max_size as usize } }
    fn push(&mut self, x: i32) { if self.black_stack.len() < self.black_size { self.black_stack.push(x); } }
    fn pop(&mut self) -> i32 { let i = self.black_stack.len() as i32 - 1; if i < 0 { return -1; } if i > 0 { self.black_inc[i as usize - 1] += self.black_inc[i as usize]; } let res = self.black_stack.pop().unwrap() + self.black_inc[i as usize]; self.black_inc[i as usize] = 0; res }
    fn increment(&mut self, k: i32, val: i32) { let i = (k as usize).min(self.black_stack.len()).saturating_sub(1); if !self.black_stack.is_empty() { self.black_inc[i] += val; } }
}