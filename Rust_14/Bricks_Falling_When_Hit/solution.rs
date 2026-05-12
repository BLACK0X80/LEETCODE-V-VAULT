impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let top = m * n;
        let mut par: Vec<usize> = (0..=top).collect();
        let mut rnk = vec![0u8; top + 1];
        let mut sz = vec![1usize; top + 1];

        fn find(par: &mut Vec<usize>, x: usize) -> usize {
            if par[x] != x { par[x] = find(par, par[x]); }
            par[x]
        }
        fn union(par: &mut Vec<usize>, rnk: &mut Vec<u8>, sz: &mut Vec<usize>, x: usize, y: usize) {
            let (mut rx, mut ry) = (find(par, x), find(par, y));
            if rx == ry { return; }
            if rnk[rx] < rnk[ry] { std::mem::swap(&mut rx, &mut ry); }
            if rnk[rx] == rnk[ry] { rnk[rx] += 1; }
            par[ry] = rx;
            sz[rx] += sz[ry];
        }

        let idx = |r: usize, c: usize| r * n + c;
        let mut a = grid.clone();
        for h in &hits { a[h[0] as usize][h[1] as usize] = 0; }

        for r in 0..m {
            for c in 0..n {
                if a[r][c] == 1 {
                    let i = idx(r, c);
                    if r == 0 { union(&mut par, &mut rnk, &mut sz, i, top); }
                    if r > 0 && a[r-1][c] == 1 { union(&mut par, &mut rnk, &mut sz, i, idx(r-1,c)); }
                    if c > 0 && a[r][c-1] == 1 { union(&mut par, &mut rnk, &mut sz, i, idx(r,c-1)); }
                }
            }
        }

        let mut ans = vec![0i32; hits.len()];
        for i in (0..hits.len()).rev() {
            let (r, c) = (hits[i][0] as usize, hits[i][1] as usize);
            let pre = sz[find(&mut par, top)];
            if grid[r][c] == 0 { continue; }
            let ci = idx(r, c);
            for (dr, dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr = r as i32 + dr; let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 { continue; }
                let (nr, nc) = (nr as usize, nc as usize);
                if a[nr][nc] == 1 { union(&mut par, &mut rnk, &mut sz, ci, idx(nr,nc)); }
            }
            if r == 0 { union(&mut par, &mut rnk, &mut sz, ci, top); }
            a[r][c] = 1;
            ans[i] = (sz[find(&mut par, top)] as i32 - pre as i32 - 1).max(0);
        }
        ans
    }
}