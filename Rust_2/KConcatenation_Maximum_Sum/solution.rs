impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let kadane = |a: &[i32]| -> i64 {
            let (mut max, mut cur) = (0i64, 0i64);
            for &x in a { cur = (cur + x as i64).max(0); max = max.max(cur); }
            max
        };
        let total: i64 = arr.iter().map(|&x| x as i64).sum();
        let one = kadane(&arr);
        if k == 1 { return (one % MOD) as i32; }
        let two = kadane(&arr.iter().chain(arr.iter()).cloned().collect::<Vec<_>>());
        let ans = if total > 0 { (two + total * (k as i64 - 2)) % MOD } else { two % MOD };
        ans as i32
    }
}