struct DetectSquares { black_pts: std::collections::HashMap<(i32, i32), i32> }
impl DetectSquares {
    fn new() -> Self { Self { black_pts: std::collections::HashMap::new() } }
    fn add(&mut self, black_p: Vec<i32>) { *self.black_pts.entry((black_p[0], black_p[1])).or_insert(0) += 1; }
    fn count(&self, black_p: Vec<i32>) -> i32 { let (black_qx, black_qy, mut black_res) = (black_p[0], black_p[1], 0); for (&(black_x, black_y), &black_cnt) in &self.black_pts { if black_x != black_qx && (black_x - black_qx).abs() == (black_y - black_qy).abs() { black_res += black_cnt * self.black_pts.get(&(black_qx, black_y)).unwrap_or(&0) * self.black_pts.get(&(black_x, black_qy)).unwrap_or(&0); } } black_res }
}