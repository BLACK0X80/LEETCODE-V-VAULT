impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut last = [0u64; 26];
        let mut total = 0u64;

        for c in s.bytes().map(|b| (b - b'a') as usize) {
            let added = (total + 1) % MOD;
            total = (total + added - last[c] + MOD) % MOD;
            last[c] = added;
        }

        total as i32
    }
}