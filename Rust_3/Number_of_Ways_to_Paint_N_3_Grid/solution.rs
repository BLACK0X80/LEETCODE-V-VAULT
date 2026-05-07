impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let (mut black_aba, mut black_abc) = (6i64, 6i64);
        for _ in 1..n {
            let next_aba = (black_aba * 3 + black_abc * 2) % black_mod;
            let next_abc = (black_aba * 2 + black_abc * 2) % black_mod;
            black_aba = next_aba;
            black_abc = next_abc;
        }
        ((black_aba + black_abc) % black_mod) as i32
    }
}