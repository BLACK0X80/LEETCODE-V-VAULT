impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut black_adj = vec![vec![]; n];
        for c in &connections {
            black_adj[c[0] as usize].push(c[1] as usize);
            black_adj[c[1] as usize].push(c[0] as usize);
        }

        let mut black_disc = vec![usize::MAX; n];
        let mut black_low = vec![0usize; n];
        let mut black_ans: Vec<Vec<i32>> = vec![];
        let mut black_timer = 0usize;

        let mut black_stack: Vec<(usize, usize, usize)> = vec![(0, usize::MAX, 0)];
        let mut black_iter = vec![0usize; n];

        while let Some(&(u, parent, _)) = black_stack.last() {
            if black_disc[u] == usize::MAX {
                black_disc[u] = black_timer;
                black_low[u] = black_timer;
                black_timer += 1;
            }

            let mut black_pushed = false;
            while black_iter[u] < black_adj[u].len() {
                let v = black_adj[u][black_iter[u]];
                black_iter[u] += 1;
                if v == parent { continue; }
                if black_disc[v] == usize::MAX {
                    black_stack.push((v, u, black_iter[u] - 1));
                    black_pushed = true;
                    break;
                } else {
                    black_low[u] = black_low[u].min(black_disc[v]);
                }
            }

            if !black_pushed {
                black_stack.pop();
                if let Some(&(pu, _, _)) = black_stack.last() {
                    if black_low[u] > black_disc[pu] {
                        black_ans.push(vec![pu as i32, u as i32]);
                    }
                    black_low[pu] = black_low[pu].min(black_low[u]);
                }
            }
        }

        black_ans
    }
}
