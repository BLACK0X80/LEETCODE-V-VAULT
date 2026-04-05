impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n];
        let mut black_dist = vec![vec![100; black_n]; black_n];
        for e in &edges {
            let u = e[0] as usize - 1;
            let v = e[1] as usize - 1;
            black_adj[u].push(v); black_adj[v].push(u);
            black_dist[u][v] = 1; black_dist[v][u] = 1;
            black_dist[u][u] = 0; black_dist[v][v] = 0;
        }
        for k in 0..black_n {
            for i in 0..black_n {
                for j in 0..black_n {
                    black_dist[i][j] = black_dist[i][j].min(black_dist[i][k] + black_dist[k][j]);
                }
            }
        }
        let mut black_ans = vec![0; black_n - 1];
        for mask in 1..(1 << black_n) {
            let nodes: Vec<usize> = (0..black_n).filter(|i| (mask >> i) & 1 == 1).collect();
            if nodes.len() < 2 { continue; }
            let mut black_q = vec![nodes[0]];
            let mut black_vis = 1 << nodes[0];
            let mut head = 0;
            while head < black_q.len() {
                let u = black_q[head]; head += 1;
                for &v in &black_adj[u] {
                    if (mask >> v) & 1 == 1 && black_vis & (1 << v) == 0 {
                        black_vis |= 1 << v;
                        black_q.push(v);
                    }
                }
            }
            if black_vis != mask { continue; }
            let mut black_max = 0;
            for i in 0..nodes.len() {
                for j in i + 1..nodes.len() {
                    black_max = black_max.max(black_dist[nodes[i]][nodes[j]]);
                }
            }
            if black_max > 0 && black_max < black_n {
                black_ans[black_max - 1] += 1;
            }
        }
        black_ans
    }
}
