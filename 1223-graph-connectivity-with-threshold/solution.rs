impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut black_dsu: Vec<usize> = (0..=n).collect();
        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }
        for z in (threshold as usize + 1)..=n {
            for mul in (2*z..=n).step_by(z) {
                let (pz, pm) = (black_find(&mut black_dsu, z), black_find(&mut black_dsu, mul));
                if pz != pm { black_dsu[pz] = pm; }
            }
        }
        queries.iter().map(|q| black_find(&mut black_dsu, q[0] as usize) == black_find(&mut black_dsu, q[1] as usize)).collect()
    }
}
