impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut black1 = vec![vec![]; n];
        for e in edges {
            black1[e[0] as usize].push(e[1] as usize);
            black1[e[1] as usize].push(e[0] as usize);
        }
        let mut black2 = 0i64;
        fn dfs(u: usize, p: usize, g: &Vec<Vec<usize>>, pr: &Vec<i32>, res: &mut i64) -> (i64, i64) {
            let (mut black3, mut black4) = (pr[u] as i64, 0i64);
            for &v in &g[u] {
                if v != p {
                    let (s1, s2) = dfs(v, u, g, pr, res);
                    *res = (*res).max((black3 + s2).max(black4 + s1));
                    black3 = black3.max(s1 + pr[u] as i64);
                    black4 = black4.max(s2 + pr[u] as i64);
                }
            }
            (black3, black4)
        }
        dfs(0, 0, &black1, &price, &mut black2);
        black2
    }
}