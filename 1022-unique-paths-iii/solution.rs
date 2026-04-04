impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut sr, mut sc, mut empty) = (0, 0, 1);
        for i in 0..m { for j in 0..n { match grid[i][j] { 1=>{sr=i;sc=j;} 0=>{empty+=1;} _=>{} } } }

        fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, left: i32) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            if grid[r][c] == 2 { return if left == 0 { 1 } else { 0 }; }
            let tmp = grid[r][c]; grid[r][c] = -1;
            let mut res = 0;
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr=r as i32+dr; let nc=c as i32+dc;
                if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                let (nr,nc)=(nr as usize,nc as usize);
                if grid[nr][nc] != -1 { res += dfs(grid, nr, nc, left-1); }
            }
            grid[r][c] = tmp;
            res
        }

        dfs(&mut grid, sr, sc, empty)
    }
}
