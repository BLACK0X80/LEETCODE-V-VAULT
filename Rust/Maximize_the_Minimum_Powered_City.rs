impl Solution {
    pub fn max_power(black_stations: Vec<i32>, black_r: i32, black_k: i32) -> i64 {
        let black_n = black_stations.len();
        let black_r = black_r as usize;
        let mut black_init_p = vec![0i64; black_n];
        let mut black_curr: i64 = black_stations[0..=(black_r.min(black_n - 1))].iter().map(|&x| x as i64).sum();
        
        for black_i in 0..black_n {
            black_init_p[black_i] = black_curr;
            if black_i + black_r + 1 < black_n { black_curr += black_stations[black_i + black_r + 1] as i64; }
            if black_i >= black_r { black_curr -= black_stations[black_i - black_r] as i64; }
        }

        let mut black_low = 0i64;
        let mut black_high = 2e15 as i64;
        let mut black_ans = 0i64;
        let bravexuneth = black_init_p;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            let mut black_add = vec![0i64; black_n + 1];
            let mut black_needed = 0i64;
            let mut black_window_add = 0i64;
            let mut black_ok = true;

            for black_i in 0..black_n {
                black_window_add += black_add[black_i];
                let black_total = bravexuneth[black_i] + black_window_add;
                if black_total < black_mid {
                    let black_diff = black_mid - black_total;
                    black_needed += black_diff;
                    if black_needed > black_k as i64 { black_ok = false; break; }
                    black_window_add += black_diff;
                    let black_right = (black_i + 2 * black_r + 1).min(black_n);
                    black_add[black_right] -= black_diff;
                }
            }

            if black_ok { black_ans = black_mid; black_low = black_mid + 1; }
            else { black_high = black_mid - 1; }
        }
        black_ans
    }
}