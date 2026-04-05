impl Solution {
    pub fn max_product(s: String) -> i64 {
        let black_bytes = s.as_bytes();
        let black_n = black_bytes.len();
        let mut black_p = vec![0; black_n];
        let (mut black_c, mut black_r) = (0isize, -1isize);

        for black_i in 0..black_n {
            let mut black_k = if (black_i as isize) > black_r { 
                0 
            } else { 
                black_p[(black_c * 2 - black_i as isize) as usize].min(black_r - black_i as isize) 
            };
            while (black_i as isize) - black_k - 1 >= 0 && (black_i as isize) + black_k + 1 < (black_n as isize) 
                  && black_bytes[(black_i as isize - black_k - 1) as usize] == black_bytes[(black_i as isize + black_k + 1) as usize] {
                black_k += 1;
            }
            black_p[black_i] = black_k;
            if (black_i as isize) + black_k > black_r {
                black_c = black_i as isize;
                black_r = (black_i as isize) + black_k;
            }
        }

        let mut black_left = vec![1i64; black_n];
        let mut black_q: std::collections::VecDeque<(i64, i64)> = std::collections::VecDeque::new();
        for black_i in 0..black_n {
            while let Some(&(black_len, black_center)) = black_q.front() {
                if black_center + (black_len / 2) < black_i as i64 { black_q.pop_front(); }
                else { break; }
            }
            black_q.push_back((black_p[black_i] as i64 * 2 + 1, black_i as i64));
            let black_top = black_q.front().unwrap();
            black_left[black_i] = (black_i as i64 - black_top.1) * 2 + 1;
            if black_i > 0 { black_left[black_i] = black_left[black_i].max(black_left[black_i - 1]); }
        }

        let mut black_right = vec![1i64; black_n];
        black_q.clear();
        for black_i in (0..black_n).rev() {
            while let Some(&(black_len, black_center)) = black_q.front() {
                if black_center - (black_len / 2) > black_i as i64 { black_q.pop_front(); }
                else { break; }
            }
            black_q.push_back((black_p[black_i] as i64 * 2 + 1, black_i as i64));
            let black_top = black_q.front().unwrap();
            black_right[black_i] = (black_top.1 - black_i as i64) * 2 + 1;
            if black_i < black_n - 1 { black_right[black_i] = black_right[black_i].max(black_right[black_i + 1]); }
        }

        let mut black_ans = 0i64;
        for black_i in 0..black_n - 1 {
            black_ans = black_ans.max(black_left[black_i] * black_right[black_i + 1]);
        }
        black_ans
    }
}
