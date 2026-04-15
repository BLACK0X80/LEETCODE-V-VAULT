impl Solution {
    pub fn min_length(black_s: String, black_num_ops: i32) -> i32 {
        let black_b = black_s.as_bytes();
        let black_n = black_b.len();
        let mut black_check = |black_mid: i32| -> bool {
            if black_mid == 1 {
                let mut black_c0 = 0;
                let mut black_c1 = 0;
                for i in 0..black_n {
                    if black_b[i] != (if i % 2 == 0 { b'0' } else { b'1' }) { black_c0 += 1; }
                    if black_b[i] != (if i % 2 == 0 { b'1' } else { b'0' }) { black_c1 += 1; }
                }
                return black_c0.min(black_c1) <= black_num_ops;
            }
            let (mut black_ops, mut black_cnt) = (0, 1);
            for i in 1..black_n {
                if black_b[i] == black_b[i-1] { black_cnt += 1; }
                else { black_ops += black_cnt / (black_mid + 1); black_cnt = 1; }
            }
            black_ops += black_cnt / (black_mid + 1);
            black_ops <= black_num_ops
        };
        let (mut black_low, mut black_high) = (1, black_n as i32);
        let mut black_res = black_high;
        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if black_check(black_mid) { black_res = black_mid; black_high = black_mid - 1; }
            else { black_low = black_mid + 1; }
        }
        black_res
    }
}