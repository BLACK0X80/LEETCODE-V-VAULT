impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut black_parent = vec![0usize; n + 1];
        let mut black_cand1: Option<Vec<i32>> = None;
        let mut black_cand2: Option<Vec<i32>> = None;

        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            if black_parent[v] != 0 {
                black_cand1 = Some(vec![black_parent[v] as i32, v as i32]);
                black_cand2 = Some(e.clone());
            }
            black_parent[v] = u;
        }

        let mut black_dsu: Vec<usize> = (0..=n).collect();
        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }

        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            if black_cand2.as_ref().map_or(false, |c| c == e) { continue; }
            let (pu, pv) = (black_find(&mut black_dsu, u), black_find(&mut black_dsu, v));
            if pu == pv { return black_cand1.unwrap_or_else(|| e.clone()); }
            black_dsu[pu] = pv;
        }
        black_cand2.unwrap()
    }
}
