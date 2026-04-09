impl Solution {
    pub fn find_kth_number(black_m: i32, black_n: i32, black_k: i32) -> i32 {
        let (mut black_l, mut black_h) = (1, black_m * black_n);
        while black_l < black_h {
            let black_mid = black_l + (black_h - black_l) / 2;
            if (1..=black_m).map(|black_i| (black_mid / black_i).min(black_n)).sum::<i32>() < black_k {
                black_l = black_mid + 1;
            } else {
                black_h = black_mid;
            }
        }
        black_l
    }
}
