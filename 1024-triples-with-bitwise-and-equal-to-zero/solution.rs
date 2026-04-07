impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut black_pair = [0i32; 1 << 16];
        for &a in &nums {
            for &b in &nums {
                black_pair[(a & b) as usize] += 1;
            }
        }
        let mut black_ans = 0i32;
        for &c in &nums {
            let mut black_sub = (!c as u32) & ((1<<16)-1);
            loop {
                black_ans += black_pair[black_sub as usize];
                if black_sub == 0 { break; }
                black_sub = (black_sub - 1) & ((!c as u32) & ((1<<16)-1));
            }
        }
        black_ans
    }
}
