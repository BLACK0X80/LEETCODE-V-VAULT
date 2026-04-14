impl Solution {
    pub fn median_of_uniqueness_array(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let black_total = black_n as i64 * (black_n as i64 + 1) / 2;
        let black_check = |black_m: i32| {
            let (mut black_l, mut black_count, mut black_map) = (0, 0i64, std::collections::HashMap::new());
            for black_r in 0..black_n {
                *black_map.entry(black_nums[black_r]).or_insert(0) += 1;
                while black_map.len() > black_m as usize {
                    let black_v = black_map.get_mut(&black_nums[black_l]).unwrap(); *black_v -= 1;
                    if *black_v == 0 { black_map.remove(&black_nums[black_l]); }
                    black_l += 1;
                }
                black_count += (black_r - black_l + 1) as i64;
            }
            black_count >= (black_total + 1) / 2
        };
        let (mut black_low, mut black_high, mut black_res) = (1, black_n as i32, 1);
        while black_low <= black_high {
            let black_mid = (black_low + black_high) / 2;
            if black_check(black_mid) { black_res = black_mid; black_high = black_mid - 1; } else { black_low = black_mid + 1; }
        }
        black_res
    }
}