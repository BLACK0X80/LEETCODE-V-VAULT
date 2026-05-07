struct StockSpanner { black_stack: Vec<(i32, i32)> }
impl StockSpanner {
    fn new() -> Self { Self { black_stack: Vec::new() } }
    fn next(&mut self, price: i32) -> i32 { let mut black_res = 1; while let Some(&(black_p, black_s)) = self.black_stack.last() { if price >= black_p { black_res += black_s; self.black_stack.pop(); } else { break; } } self.black_stack.push((price, black_res)); black_res }
}