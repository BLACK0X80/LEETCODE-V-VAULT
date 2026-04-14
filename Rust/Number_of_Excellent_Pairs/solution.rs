use std::collections::HashSet;
impl Solution {
    pub fn count_excellent_pairs(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let mut black_set: HashSet<i32> = black_nums.into_iter().collect();
        let mut black_cnt = vec![0i64; 31];
        for black_x in black_set { black_cnt[black_x.count_ones() as usize] += 1; }
        let mut black_ans = 0i64;
        for i in 1..31 {
            for j in 1..31 {
                if i + j >= black_k as usize { black_ans += black_cnt[i] * black_cnt[j]; }
            }
        }
        black_ans
    }
}