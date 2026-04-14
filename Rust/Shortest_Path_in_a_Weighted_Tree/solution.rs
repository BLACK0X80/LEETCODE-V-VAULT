struct Fenwick {
    black_tree: Vec<i64>,
}
impl Fenwick {
    fn new(black_n: usize) -> Self { Self { black_tree: vec![0; black_n + 1] } }
    fn add(&mut self, mut black_i: usize, black_val: i64) {
        black_i += 1;
        while black_i < self.black_tree.len() {
            self.black_tree[black_i] += black_val;
            black_i += (black_i as isize & -(black_i as isize)) as usize;
        }
    }
    fn query(&self, mut black_i: usize) -> i64 {
        black_i += 1;
        let mut black_s = 0;
        while black_i > 0 {
            black_s += self.black_tree[black_i];
            black_i -= (black_i as isize & -(black_i as isize)) as usize;
        }
        black_s
    }
}
impl Solution {
    pub fn tree_queries(black_n: i32, black_edges: Vec<Vec<i32>>, black_queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = black_n as usize;
        let mut black_adj = vec![Vec::new(); black_n + 1];
        let mut black_edge_map = std::collections::HashMap::new();
        for (black_idx, black_e) in black_edges.iter().enumerate() {
            black_adj[black_e[0] as usize].push((black_e[1] as usize, black_e[2] as i64));
            black_adj[black_e[1] as usize].push((black_e[0] as usize, black_e[2] as i64));
            let mut black_key = [black_e[0], black_e[1]];
            black_key.sort();
            black_edge_map.insert((black_key[0], black_key[1]), (black_idx, black_e[2] as i64));
        }
        let (mut black_tin, mut black_tout, mut black_timer) = (vec![0; black_n + 1], vec![0; black_n + 1], 0);
        let mut black_edge_to_child = vec![0; black_n - 1];
        Self::dfs(1, 0, &black_adj, &mut black_tin, &mut black_tout, &mut black_timer, &mut black_edge_to_child, &black_edge_map);
        let mut black_bit = Fenwick::new(black_n + 1);
        let mut black_cur_weights = vec![0i64; black_n - 1];
        for (black_key, &(black_idx, black_w)) in &black_edge_map {
            black_cur_weights[black_idx] = black_w;
            let black_child = black_edge_to_child[black_idx];
            black_bit.add(black_tin[black_child], black_w);
            black_bit.add(black_tout[black_child] + 1, -black_w);
        }
        let mut black_ans = Vec::new();
        for black_q in black_queries {
            if black_q[0] == 1 {
                let mut black_key = [black_q[1], black_q[2]];
                black_key.sort();
                let &(black_idx, _) = black_edge_map.get(&(black_key[0], black_key[1])).unwrap();
                let black_diff = black_q[3] as i64 - black_cur_weights[black_idx];
                black_cur_weights[black_idx] = black_q[3] as i64;
                let black_child = black_edge_to_child[black_idx];
                black_bit.add(black_tin[black_child], black_diff);
                black_bit.add(black_tout[black_child] + 1, -black_diff);
            } else {
                black_ans.push(black_bit.query(black_tin[black_q[1] as usize]));
            }
        }
        black_ans
    }
    fn dfs(black_u: usize, black_p: usize, black_adj: &Vec<Vec<(usize, i64)>>, black_tin: &mut Vec<usize>, black_tout: &mut Vec<usize>, black_timer: &mut usize, black_etc: &mut Vec<usize>, black_map: &std::collections::HashMap<(i32, i32), (usize, i64)>) {
        black_tin[black_u] = *black_timer; *black_timer += 1;
        for &(black_v, _) in &black_adj[black_u] {
            if black_v != black_p {
                let mut black_key = [black_u as i32, black_v as i32];
                black_key.sort();
                let &(black_idx, _) = black_map.get(&(black_key[0], black_key[1])).unwrap();
                black_etc[black_idx] = black_v;
                Self::dfs(black_v, black_u, black_adj, black_tin, black_tout, black_timer, black_etc, black_map);
            }
        }
        black_tout[black_u] = *black_timer - 1;
    }
}