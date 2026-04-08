use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let black_p = black_nums.iter().position(|&x| x == black_k).unwrap();
        let mut black_cnt = HashMap::new();
        let (mut black_ans, mut black_bal) = (0, 0);
        black_cnt.insert(0, 1);
        for i in (0..black_p).rev() {
            black_bal += if black_nums[i] > black_k { 1 } else { -1 };
            *black_cnt.entry(black_bal).or_insert(0) += 1;
        }
        black_bal = 0;
        for i in black_p..black_nums.len() {
            if i > black_p { black_bal += if black_nums[i] > black_k { 1 } else { -1 }; }
            black_ans += black_cnt.get(&(-black_bal)).unwrap_or(&0);
            black_ans += black_cnt.get(&(1 - black_bal)).unwrap_or(&0);
        }
        black_ans
    }
}
