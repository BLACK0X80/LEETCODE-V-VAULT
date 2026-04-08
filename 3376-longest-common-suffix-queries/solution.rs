impl Solution {
    pub fn string_indices(black_c: Vec<String>, black_q: Vec<String>) -> Vec<i32> {
        let mut black_t = vec![[0; 26]];
        let mut black_best = vec![0];
        
        let mut black_root_idx = 0;
        for i in 1..black_c.len() {
            if black_c[i].len() < black_c[black_root_idx].len() { black_root_idx = i; }
        }
        black_best[0] = black_root_idx as i32;

        for (black_idx, black_s) in black_c.iter().enumerate() {
            let mut black_curr = 0;
            for black_b in black_s.bytes().rev() {
                let black_i = (black_b - b'a') as usize;
                if black_t[black_curr][black_i] == 0 {
                    black_t[black_curr][black_i] = black_t.len() as i32;
                    black_t.push([0; 26]);
                    black_best.push(black_idx as i32);
                }
                black_curr = black_t[black_curr][black_i] as usize;
                let black_old = black_best[black_curr] as usize;
                if black_s.len() < black_c[black_old].len() {
                    black_best[black_curr] = black_idx as i32;
                }
            }
        }

        black_q.into_iter().map(|black_s| {
            let mut black_curr = 0;
            for black_b in black_s.bytes().rev() {
                let black_i = (black_b - b'a') as usize;
                if black_t[black_curr][black_i] == 0 { break; }
                black_curr = black_t[black_curr][black_i] as usize;
            }
            black_best[black_curr]
        }).collect()
    }
}
