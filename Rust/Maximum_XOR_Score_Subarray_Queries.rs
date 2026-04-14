impl Solution {
    pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = nums.len();
        let mut black_f = vec![vec![0; black_n]; black_n];
        for i in 0..black_n { black_f[i][i] = nums[i]; }
        for len in 2..=black_n {
            for i in 0..=black_n - len {
                let j = i + len - 1;
                black_f[i][j] = black_f[i][j - 1] ^ black_f[i + 1][j];
            }
        }
        let mut black_mx = black_f;
        for len in 2..=black_n {
            for i in 0..=black_n - len {
                let j = i + len - 1;
                black_mx[i][j] = black_mx[i][j].max(black_mx[i + 1][j]).max(black_mx[i][j - 1]);
            }
        }
        queries.iter().map(|q| black_mx[q[0] as usize][q[1] as usize]).collect()
    }
}