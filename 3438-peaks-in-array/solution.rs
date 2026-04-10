struct BlackBIT { black_tree: Vec<i32> }
impl BlackBIT {
    fn new(black_n: usize) -> Self { Self { black_tree: vec![0; black_n + 1] } }
    fn update(&mut self, mut black_i: usize, black_val: i32) {
        black_i += 1;
        while black_i < self.black_tree.len() { self.black_tree[black_i] += black_val; black_i += (black_i as i32 & -(black_i as i32)) as usize; }
    }
    fn query(&self, mut black_i: usize) -> i32 {
        let mut black_sum = 0; black_i += 1;
        while black_i > 0 { black_sum += self.black_tree[black_i]; black_i -= (black_i as i32 & -(black_i as i32)) as usize; }
        black_sum
    }
}
impl Solution {
    pub fn count_of_peaks(mut black_nums: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_bit = BlackBIT::new(black_n);
        let mut black_is_peak = vec![false; black_n];
        let black_check = |black_i: usize, black_v: &[i32]| {
            if black_i <= 0 || black_i >= black_v.len() - 1 { false }
            else { black_v[black_i] > black_v[black_i-1] && black_v[black_i] > black_v[black_i+1] }
        };
        for black_i in 1..black_n-1 { if black_check(black_i, &black_nums) { black_is_peak[black_i] = true; black_bit.update(black_i, 1); } }
        let mut black_ans = vec![];
        for black_q in black_queries {
            if black_q[0] == 1 {
                let (black_l, black_r) = (black_q[1] as usize, black_q[2] as usize);
                if black_r - black_l < 2 { black_ans.push(0); }
                else { black_ans.push(black_bit.query(black_r - 1) - black_bit.query(black_l)); }
            } else {
                let (black_idx, black_val) = (black_q[1] as usize, black_q[2]);
                for black_j in black_idx.saturating_sub(1)..=(black_idx + 1).min(black_n - 1) {
                    if black_is_peak[black_j] { black_bit.update(black_j, -1); black_is_peak[black_j] = false; }
                }
                black_nums[black_idx] = black_val;
                for black_j in black_idx.saturating_sub(1)..=(black_idx + 1).min(black_n - 1) {
                    if black_check(black_j, &black_nums) { black_is_peak[black_j] = true; black_bit.update(black_j, 1); }
                }
            }
        }
        black_ans
    }
}
