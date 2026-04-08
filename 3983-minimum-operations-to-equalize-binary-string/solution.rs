use std::collections::VecDeque;

impl Solution {
    pub fn min_operations(black_s: String, black_k: i32) -> i32 {
        let black_n = black_s.len();
        let black_k = black_k as usize;
        let black_z = black_s.chars().filter(|&c| c == '0').count();

        if black_z == 0 { return 0; }

        let mut black_q = VecDeque::from([black_z]);
        let mut black_v = vec![-1; black_n + 1];
        black_v[black_z] = 0;

        while let Some(black_u) = black_q.pop_front() {
            let black_o = black_n - black_u;
            
            let black_min_i = if black_k > black_o { black_k - black_o } else { 0 };
            let black_max_i = black_u.min(black_k);
            
            let black_low = black_u + black_k - 2 * black_max_i;
            let black_high = black_u + black_k - 2 * black_min_i;

            let mut black_next = black_low;
            while black_next <= black_high {
                if black_v[black_next] == -1 {
                    black_v[black_next] = black_v[black_u] + 1;
                    if black_next == 0 { return black_v[black_next]; }
                    black_q.push_back(black_next);
                }
                black_next += 2;
            }
        }
        black_v[0]
    }
}
