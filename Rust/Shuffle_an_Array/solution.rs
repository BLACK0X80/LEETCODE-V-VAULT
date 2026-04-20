struct Solution { black_v: Vec<i32>, black_s: std::cell::Cell<u64> }

impl Solution {
    fn new(black: Vec<i32>) -> Self { Self { black_v: black, black_s: std::cell::Cell::new(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64 | 1) } }

    fn reset(&self) -> Vec<i32> { self.black_v.clone() }

    fn shuffle(&self) -> Vec<i32> { let mut black_res = self.black_v.clone(); for black_i in (0..black_res.len()).rev() { let mut black_x = self.black_s.get(); black_x ^= black_x << 13; black_x ^= black_x >> 7; black_x ^= black_x << 17; self.black_s.set(black_x); black_res.swap(black_i, (black_x as usize % (black_i + 1))); } black_res }
}