impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let (n, k) = (n as usize, k as usize);
        let mut black = vec![vec![0u64; k + 1]; n + 1];
        black[0][0] = 1;
        for b in 1..=n {
            for bl in 1..=k {
                black[b][bl] = (black[b-1][bl-1] + (b as u64 - 1) * black[b-1][bl]) % MOD;
            }
        }
        black[n][k] as i32
    }
}
