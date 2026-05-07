impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let black_m = m as usize;
        let mut black_states = Vec::new();

        fn black_gen(idx: usize, current: Vec<i32>, m: usize, res: &mut Vec<Vec<i32>>) {
            if idx == m { res.push(current); return; }
            for c in 0..3 {
                if idx > 0 && current[idx - 1] == c { continue; }
                let mut next = current.clone();
                next.push(c);
                black_gen(idx + 1, next, m, res);
            }
        }
        black_gen(0, vec![], black_m, &mut black_states);

        let black_sz = black_states.len();
        let mut black_adj = vec![vec![]; black_sz];
        for i in 0..black_sz {
            for j in 0..black_sz {
                let mut ok = true;
                for k in 0..black_m {
                    if black_states[i][k] == black_states[j][k] { ok = false; break; }
                }
                if ok { black_adj[i].push(j); }
            }
        }

        let mut black_dp = vec![1i64; black_sz];
        for _ in 1..n {
            let mut black_next = vec![0i64; black_sz];
            for i in 0..black_sz {
                for &next_idx in &black_adj[i] {
                    black_next[next_idx] = (black_next[next_idx] + black_dp[i]) % black_mod;
                }
            }
            black_dp = black_next;
        }

        (black_dp.iter().sum::<i64>() % black_mod) as i32
    }
}