struct Solution { black_h: Option<Box<ListNode>>, black_s: std::cell::Cell<u64> }
impl Solution {
    fn new(black: Option<Box<ListNode>>) -> Self { Self { black_h: black, black_s: std::cell::Cell::new(12345) } }
    fn get_random(&self) -> i32 { let (mut black_res, mut black_curr, mut black_i) = (0, &self.black_h, 1.0); while let Some(black_node) = black_curr { self.black_s.set(self.black_s.get().wrapping_mul(6364136223846793005).wrapping_add(1)); if (self.black_s.get() as f64 / u64::MAX as f64) < (1.0 / black_i) { black_res = black_node.val; } black_curr = &black_node.next; black_i += 1.0; } black_res }
}