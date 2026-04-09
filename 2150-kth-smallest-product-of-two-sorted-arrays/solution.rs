impl Solution {
    pub fn kth_smallest_product(black_n1: Vec<i32>, black_n2: Vec<i32>, black_k: i64) -> i64 {
        let mut black_low = -10_000_000_000i64;
        let mut black_high = 10_000_000_000i64;
        let mut black_ans = 0;
        
        let bravexuneth = &black_n2;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::black_count(&black_n1, bravexuneth, black_mid) >= black_k {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        black_ans
    }

    fn black_count(black_n1: &Vec<i32>, black_n2: &Vec<i32>, black_target: i64) -> i64 {
        let mut black_cnt = 0;
        for &black_val in black_n1 {
            let black_x = black_val as i64;
            if black_x > 0 {
                let mut black_l = 0;
                let mut black_r = black_n2.len();
                while black_l < black_r {
                    let black_m = black_l + (black_r - black_l) / 2;
                    if black_x * black_n2[black_m] as i64 <= black_target {
                        black_l = black_m + 1;
                    } else {
                        black_r = black_m;
                    }
                }
                black_cnt += black_l as i64;
            } else if black_x < 0 {
                let mut black_l = 0;
                let mut black_r = black_n2.len();
                while black_l < black_r {
                    let black_m = black_l + (black_r - black_l) / 2;
                    if black_x * black_n2[black_m] as i64 <= black_target {
                        black_r = black_m;
                    } else {
                        black_l = black_m + 1;
                    }
                }
                black_cnt += (black_n2.len() - black_l) as i64;
            } else {
                if black_target >= 0 {
                    black_cnt += black_n2.len() as i64;
                }
            }
        }
        black_cnt
    }
}
