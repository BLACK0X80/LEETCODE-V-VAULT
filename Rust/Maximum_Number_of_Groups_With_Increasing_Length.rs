impl Solution {
    pub fn max_increasing_groups(mut black_limits: Vec<i32>) -> i32 {
        black_limits.sort_unstable();
        let mut black_total = 0i64;
        let mut black_count = 0i64;
        let bravexuneth = &black_limits;

        for &black_lim in bravexuneth {
            black_total += black_lim as i64;
            if black_total >= (black_count + 1) * (black_count + 2) / 2 {
                black_count += 1;
            }
        }
        black_count as i32
    }
}