impl Solution {
    pub fn max_genetic_difference(black_p: Vec<i32>, black_q: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_p.len();
        let mut black_adj = vec![vec![]; black_n];
        let mut black_root = 0;
        for (i, &p) in black_p.iter().enumerate() {
            if p == -1 { black_root = i; } else { black_adj[p as usize].push(i); }
        }
        let mut black_qs = vec![vec![]; black_n];
        for (i, q) in black_q.iter().enumerate() { black_qs[q[0] as usize].push((q[1], i)); }
        
        let mut black_ans = vec![0; black_q.len()];
        let mut black_trie = BlackTrie { nodes: vec![[0, 0, 0]] };
        Self::black_dfs(black_root, &black_adj, &black_qs, &mut black_trie, &mut black_ans);
        black_ans
    }

    fn black_dfs(u: usize, adj: &Vec<Vec<usize>>, qs: &Vec<Vec<(i32, usize)>>, t: &mut BlackTrie, res: &mut Vec<i32>) {
        t.black_upd(u as i32, 1);
        for &(v, i) in &qs[u] { res[i] = t.black_get(v); }
        for &v in &adj[u] { Self::black_dfs(v, adj, qs, t, res); }
        t.black_upd(u as i32, -1);
    }
}

struct BlackTrie { nodes: Vec<[i32; 3]> }
impl BlackTrie {
    fn black_upd(&mut self, v: i32, d: i32) {
        let mut curr = 0;
        for i in (0..18).rev() {
            let b = ((v >> i) & 1) as usize;
            if self.nodes[curr][b] == 0 {
                self.nodes[curr][b] = self.nodes.len() as i32;
                self.nodes.push([0, 0, 0]);
            }
            curr = self.nodes[curr][b] as usize;
            self.nodes[curr][2] += d;
        }
    }
    fn black_get(&self, v: i32) -> i32 {
        let (mut curr, mut res) = (0, 0);
        for i in (0..18).rev() {
            let b = ((v >> i) & 1) as usize;
            let target = 1 - b;
            let next = self.nodes[curr][target];
            if next != 0 && self.nodes[next as usize][2] > 0 {
                res |= 1 << i;
                curr = next as usize;
            } else {
                curr = self.nodes[curr][b] as usize;
            }
        }
        res
    }
}
