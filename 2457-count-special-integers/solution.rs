impl Solution {
    pub fn count_special_numbers(black_n: i32) -> i32 {
        let black_s = black_n.to_string();
        let black_digits = black_s.as_bytes();
        let black_len = black_digits.len();
        let mut black_res = 0;

        fn black_p(black_n: i32, black_k: i32) -> i32 {
            if black_k < 0 || black_k > black_n { return 0; }
            let mut black_ans = 1;
            for black_i in 0..black_k { black_ans *= black_n - black_i; }
            black_ans
        }

        for black_i in 1..black_len { black_res += 9 * black_p(9, black_i as i32 - 1); }

        let mut black_used = [false; 10];
        for black_i in 0..black_len {
            let black_lower = if black_i == 0 { 1 } else { 0 };
            let black_upper = (black_digits[black_i] - b'0') as i32;
            for black_d in black_lower..black_upper {
                if !black_used[black_d as usize] {
                    black_res += black_p(10 - black_i as i32 - 1, black_len as i32 - black_i as i32 - 1);
                }
            }
            if black_used[black_upper as usize] { return black_res; }
            black_used[black_upper as usize] = true;
        }
        black_res + 1
    }
}
