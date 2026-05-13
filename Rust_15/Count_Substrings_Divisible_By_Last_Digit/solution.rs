impl Solution {
    pub fn count_substrings(black_s: String) -> i64 {
        let black_n = black_s.len();
        let black_b = black_s.as_bytes();
        let mut black_ans = 0i64;

        let mut black_p3 = vec![0; black_n];
        let mut black_p7 = vec![0; black_n];
        let mut black_p9 = vec![0; black_n];

        let black_first_dig = (black_b[0] - b'0') as i32;
        black_p3[0] = black_first_dig % 3;
        black_p7[0] = black_first_dig % 7;
        black_p9[0] = black_first_dig % 9;

        for black_i in 1..black_n {
            let black_dig = (black_b[black_i] - b'0') as i32;
            black_p3[black_i] = (black_p3[black_i - 1] * 10 + black_dig) % 3;
            black_p7[black_i] = (black_p7[black_i - 1] * 10 + black_dig) % 7;
            black_p9[black_i] = (black_p9[black_i - 1] * 10 + black_dig) % 9;
        }

        let mut black_freq3 = vec![0i64; 3];
        let mut black_freq9 = vec![0i64; 9];
        let mut black_freq7 = vec![vec![0i64; 7]; 6];
        let black_inv7 = [1, 5, 4, 6, 2, 3];

        for black_j in 0..black_n {
            let black_d = (black_b[black_j] - b'0') as i32;

            if black_d != 0 {
                match black_d {
                    1 | 2 | 5 => {
                        black_ans += (black_j + 1) as i64;
                    }
                    4 => {
                        if black_j == 0 {
                            black_ans += 1;
                        } else {
                            let black_num = ((black_b[black_j - 1] - b'0') as i32) * 10 + black_d;
                            black_ans += if black_num % 4 == 0 { (black_j + 1) as i64 } else { 1 };
                        }
                    }
                    8 => {
                        if black_j == 0 {
                            black_ans += 1;
                        } else if black_j == 1 {
                            let black_num = ((black_b[0] - b'0') as i32) * 10 + 8;
                            black_ans += if black_num % 8 == 0 { 2 } else { 1 };
                        } else {
                            let black_num3 = ((black_b[black_j - 2] - b'0') as i32) * 100
                                + ((black_b[black_j - 1] - b'0') as i32) * 10
                                + 8;
                            let black_num2 = ((black_b[black_j - 1] - b'0') as i32) * 10 + 8;
                            black_ans += (if black_num3 % 8 == 0 { (black_j - 1) as i64 } else { 0 })
                                + (if black_num2 % 8 == 0 { 1 } else { 0 })
                                + 1;
                        }
                    }
                    3 | 6 => {
                        black_ans += (if black_p3[black_j] == 0 { 1 } else { 0 }) + black_freq3[black_p3[black_j] as usize];
                    }
                    7 => {
                        black_ans += if black_p7[black_j] == 0 { 1 } else { 0 };
                        for black_m in 0..6 {
                            let black_idx = (black_j % 6 + 6 - black_m) % 6;
                            let black_req = (black_p7[black_j] * black_inv7[black_m]) % 7;
                            black_ans += black_freq7[black_idx][black_req as usize];
                        }
                    }
                    9 => {
                        black_ans += (if black_p9[black_j] == 0 { 1 } else { 0 }) + black_freq9[black_p9[black_j] as usize];
                    }
                    _ => {}
                }
            }
            black_freq3[black_p3[black_j] as usize] += 1;
            black_freq7[black_j % 6][black_p7[black_j] as usize] += 1;
            black_freq9[black_p9[black_j] as usize] += 1;
        }

        black_ans
    }
}