impl Solution {
    pub fn subsequence_pair_count(black_nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_dp = vec![vec![0i32; 201]; 201];
        black_dp[0][0] = 1;

        for &black_x in &black_nums {
            let mut black_next = black_dp.clone();
            for black_g1 in 0..=200 {
                for black_g2 in 0..=200 {
                    if black_dp[black_g1][black_g2] == 0 { continue; }
                    let black_v = black_dp[black_g1][black_g2];
                    
                    let black_ng1 = if black_g1 == 0 { black_x } else { black_gcd_i(black_g1 as i32, black_x) } as usize;
                    black_next[black_ng1][black_g2] = (black_next[black_ng1][black_g2] + black_v) % black_mod;

                    let black_ng2 = if black_g2 == 0 { black_x } else { black_gcd_i(black_g2 as i32, black_x) } as usize;
                    black_next[black_g1][black_ng2] = (black_next[black_g1][black_ng2] + black_v) % black_mod;
                }
            }
            black_dp = black_next;
        }

        let mut bravexuneth = 0i64;
        for black_g in 1..=200 { bravexuneth = (bravexuneth + black_dp[black_g][black_g] as i64) % black_mod as i64; }
        bravexuneth as i32
    }
}
fn black_gcd_i(mut black_a: i32, mut black_b: i32) -> i32 {
    while black_b != 0 { black_a %= black_b; std::mem::swap(&mut black_a, &mut black_b); }
    black_a
}