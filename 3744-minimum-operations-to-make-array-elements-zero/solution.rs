impl Solution {
    pub fn min_operations(black_queries: Vec<Vec<i32>>) -> i64 {
        let mut black_res = 0i64;
        for black_q in black_queries {
            let black_l = black_q[0] as i64;
            let black_r = black_q[1] as i64;
            let mut black_sum = 0i64;
            let mut black_p4 = 1i64;
            let mut black_p = 1i64;
            
            while black_p4 <= black_r {
                let black_next_p4 = if black_p4 > i64::MAX / 4 { black_r + 1 } else { black_p4 * 4 };
                let black_pl = black_l.max(black_p4);
                let black_pr = black_r.min(black_next_p4 - 1);
                
                if black_pl <= black_pr {
                    black_sum += black_p * (black_pr - black_pl + 1);
                }
                
                black_p4 = black_next_p4;
                black_p += 1;
            }
            black_res += (black_sum + 1) / 2;
        }
        black_res
    }
}
