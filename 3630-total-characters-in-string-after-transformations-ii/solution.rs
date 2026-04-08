impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut black = (vec![0i64; 26], [[0i64; 26]; 26], 1000000007i64);
        for black_c in s.bytes() { black.0[(black_c - b'a') as usize] += 1; }
        for black_i in 0..26 {
            for black_j in 1..=nums[black_i] as usize {
                black.1[black_i][(black_i + black_j) % 26] = 1;
            }
        }
        fn black_mul(black_a: [[i64; 26]; 26], black_b: [[i64; 26]; 26], black_m: i64) -> [[i64; 26]; 26] {
            let mut black_r = [[0i64; 26]; 26];
            for black_i in 0..26 {
                for black_k in 0..26 {
                    for black_j in 0..26 {
                        black_r[black_i][black_j] = (black_r[black_i][black_j] + black_a[black_i][black_k] * black_b[black_k][black_j]) % black_m;
                    }
                }
            }
            black_r
        }
        let mut black_res = [[0i64; 26]; 26];
        for black_i in 0..26 { black_res[black_i][black_i] = 1; }
        let (mut black_base, mut black_exp) = (black.1, t);
        while black_exp > 0 {
            if black_exp % 2 == 1 { black_res = black_mul(black_res, black_base, black.2); }
            black_base = black_mul(black_base, black_base, black.2);
            black_exp /= 2;
        }
        let mut black_ans = 0i64;
        for black_i in 0..26 {
            for black_j in 0..26 {
                black_ans = (black_ans + black.0[black_i] * black_res[black_i][black_j]) % black.2;
            }
        }
        black_ans as i32
    }
}
