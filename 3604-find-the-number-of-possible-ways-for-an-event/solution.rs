impl Solution {
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = n as usize;
        let black_x = x as usize;
        let black_y = y as usize;

        let mut black_s = vec![vec![0i64; black_n + 1]; black_n + 1];
        black_s[0][0] = 1;
        for i in 1..=black_n {
            for j in 1..=i {
                black_s[i][j] = (black_s[i - 1][j - 1] + j as i64 * black_s[i - 1][j]) % black_mod;
            }
        }

        let mut black_res = 0i64;
        let mut black_p_x = 1i64;
        let mut black_p_y = 1i64;

        for k in 1..=black_n.min(black_x) {
            black_p_x = (black_p_x * (black_x - k + 1) as i64) % black_mod;
            black_p_y = (black_p_y * black_y as i64) % black_mod;
            
            let black_term = (black_s[black_n][k] * black_p_x) % black_mod;
            black_res = (black_res + black_term * black_p_y) % black_mod;
        }

        black_res as i32
    }
}
