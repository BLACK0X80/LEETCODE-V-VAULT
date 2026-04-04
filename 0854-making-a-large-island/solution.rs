impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut sizes = vec![0i32; n*n+2];
        let mut id = 2;

        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, id: i32) -> i32 {
            let n = grid.len();
            if grid[i][j] != 1 { return 0; }
            grid[i][j] = id;
            let mut s = 1;
            if i>0{s+=dfs(grid,i-1,j,id);} if i+1<n{s+=dfs(grid,i+1,j,id);}
            if j>0{s+=dfs(grid,i,j-1,id);} if j+1<n{s+=dfs(grid,i,j+1,id);}
            s
        }

        for i in 0..n { for j in 0..n { if grid[i][j]==1 {
            sizes[id as usize] = dfs(&mut grid,i,j,id as i32);
            id += 1;
        }}}

        let mut ans = *sizes.iter().max().unwrap();
        for i in 0..n { for j in 0..n { if grid[i][j]==0 {
            let mut seen = std::collections::HashSet::new();
            let mut s = 1i32;
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr=i as i32+dr; let nc=j as i32+dc;
                if nr<0||nc<0||nr>=n as i32||nc>=n as i32{continue;}
                let v = grid[nr as usize][nc as usize];
                if v>=2 && seen.insert(v) { s += sizes[v as usize]; }
            }
            ans = ans.max(s);
        }}}
        ans
    }
}
