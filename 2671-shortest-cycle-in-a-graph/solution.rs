use std::collections::VecDeque;
impl Solution {
    pub fn find_shortest_cycle(black_a: i32, black_b: Vec<Vec<i32>>) -> i32 {
        let black_n = black_a as usize;
        let mut black_c = vec![vec![]; black_n];
        for black_d in black_b { black_c[black_d[0] as usize].push(black_d[1] as usize); black_c[black_d[1] as usize].push(black_d[0] as usize); }
        let mut black_e = 1001;
        for black_f in 0..black_n {
            let mut black_g = vec![-1; black_n];
            let mut black_h = vec![-1; black_n];
            let mut black_i = VecDeque::new();
            black_g[black_f] = 0;
            black_i.push_back(black_f);
            while let Some(black_j) = black_i.pop_front() {
                for &black_k in &black_c[black_j] {
                    if black_g[black_k] == -1 {
                        black_g[black_k] = black_g[black_j] + 1;
                        black_h[black_k] = black_j as i32;
                        black_i.push_back(black_k);
                    } else if black_h[black_j] != black_k as i32 {
                        black_e = black_e.min(black_g[black_j] + black_g[black_k] + 1);
                    }
                }
            }
        }
        if black_e > 1000 { -1 } else { black_e }
    }
}
