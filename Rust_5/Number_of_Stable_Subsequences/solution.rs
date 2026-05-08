impl Solution {
    pub fn count_stable_subsequences(nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_e1 = 0i64;
        let mut black_e2 = 0i64;
        let mut black_o1 = 0i64;
        let mut black_o2 = 0i64;

        for black_n in nums {
            if black_n % 2 == 0 {
                let black_new_e1 = (1 + black_o1 + black_o2 + black_e1 * 0) % black_mod;
                let black_new_e2 = black_e1;
                black_e1 = (black_e1 + black_new_e1) % black_mod;
                black_e2 = (black_e2 + black_new_e2) % black_mod;
            } else {
                let black_new_o1 = (1 + black_e1 + black_e2 + black_o1 * 0) % black_mod;
                let black_new_o2 = black_o1;
                black_o1 = (black_o1 + black_new_o1) % black_mod;
                black_o2 = (black_o2 + black_new_o2) % black_mod;
            }
        }
        ((black_e1 + black_e2 + black_o1 + black_o2) % black_mod) as i32
    }
}