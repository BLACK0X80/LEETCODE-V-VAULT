impl Solution {
    pub fn distance_limited_paths_exist(n: i32, mut edge_list: Vec<Vec<i32>>, mut queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut black_dsu: Vec<usize> = (0..n).collect();
        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }
        edge_list.sort_unstable_by_key(|e| e[2]);
        let q = queries.len();
        let mut black_idx: Vec<usize> = (0..q).collect();
        black_idx.sort_unstable_by_key(|&i| queries[i][2]);
        let mut black_ans = vec![false; q];
        let mut black_ei = 0;
        for &qi in &black_idx {
            let lim = queries[qi][2];
            while black_ei < edge_list.len() && edge_list[black_ei][2] < lim {
                let (u, v) = (edge_list[black_ei][0] as usize, edge_list[black_ei][1] as usize);
                let (pu, pv) = (black_find(&mut black_dsu, u), black_find(&mut black_dsu, v));
                if pu != pv { black_dsu[pu] = pv; }
                black_ei += 1;
            }
            let (p, q2) = (queries[qi][0] as usize, queries[qi][1] as usize);
            black_ans[qi] = black_find(&mut black_dsu, p) == black_find(&mut black_dsu, q2);
        }
        black_ans
    }
}
