impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let black_n = n as usize;
        let mut black_ans = 0;

        for black_mask in 0u32..1<<black_n {
            let mut black_dist = vec![vec![i32::MAX/2; black_n]; black_n];
            for i in 0..black_n { black_dist[i][i] = 0; }
            for r in &roads {
                let (u, v, w) = (r[0] as usize, r[1] as usize, r[2]);
                if black_mask & (1<<u) != 0 || black_mask & (1<<v) != 0 { continue; }
                black_dist[u][v] = black_dist[u][v].min(w);
                black_dist[v][u] = black_dist[v][u].min(w);
            }
            for k in 0..black_n {
                if black_mask & (1<<k) != 0 { continue; }
                for i in 0..black_n {
                    for j in 0..black_n {
                        let nd = black_dist[i][k].saturating_add(black_dist[k][j]);
                        if nd < black_dist[i][j] { black_dist[i][j] = nd; }
                    }
                }
            }
            let mut black_ok = true;
            for i in 0..black_n {
                if black_mask & (1<<i) != 0 { continue; }
                for j in 0..black_n {
                    if black_mask & (1<<j) != 0 { continue; }
                    if black_dist[i][j] > max_distance { black_ok = false; break; }
                }
                if !black_ok { break; }
            }
            if black_ok { black_ans += 1; }
        }
        black_ans
    }
}