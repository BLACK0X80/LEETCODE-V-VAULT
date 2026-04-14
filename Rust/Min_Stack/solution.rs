struct MinStack { black_s: Vec<(i32, i32)> }
impl MinStack {
    fn new() -> Self { Self { black_s: vec![] } }
    fn push(&mut self, black_v: i32) { let black_m = if let Some(&(_, black_min)) = self.black_s.last() { black_min.min(black_v) } else { black_v }; self.black_s.push((black_v, black_m)); }
    fn pop(&mut self) { self.black_s.pop(); }
    fn top(&self) -> i32 { self.black_s.last().unwrap().0 }
    fn get_min(&self) -> i32 { self.black_s.last().unwrap().1 }
}