impl Solution {
    pub fn contain_virus(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let dirs = [(0i32,1i32),(0,-1),(1,0),(-1,0)];
        let mut total_walls = 0;

        loop {
            let mut regions: Vec<(Vec<(usize,usize)>, std::collections::HashSet<(usize,usize)>, i32)> = vec![];
            let mut visited = vec![vec![false; n]; m];

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 && !visited[i][j] {
                        let mut cells = vec![];
                        let mut frontier = std::collections::HashSet::new();
                        let mut walls = 0;
                        let mut stack = vec![(i,j)];
                        while let Some((r,c)) = stack.pop() {
                            if visited[r][c] { continue; }
                            visited[r][c] = true;
                            cells.push((r,c));
                            for &(dr,dc) in &dirs {
                                let nr = r as i32+dr; let nc = c as i32+dc;
                                if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                                let (nr,nc) = (nr as usize, nc as usize);
                                if grid[nr][nc] == 1 { stack.push((nr,nc)); }
                                else if grid[nr][nc] == 0 { frontier.insert((nr,nc)); walls += 1; }
                            }
                        }
                        regions.push((cells, frontier, walls));
                    }
                }
            }

            if regions.is_empty() { break; }
            let max_idx = regions.iter().enumerate().max_by_key(|(_,r)| r.1.len()).unwrap().0;

            total_walls += regions[max_idx].2;
            for &(r,c) in &regions[max_idx].0 { grid[r][c] = -1; }

            for (idx, (_, frontier, _)) in regions.iter().enumerate() {
                if idx == max_idx { continue; }
                for &(r,c) in frontier { grid[r][c] = 1; }
            }

            if regions.iter().enumerate().filter(|&(i,_)| i != max_idx).all(|(_,(c,_,_))| c.is_empty()) && regions.len() == 1 { break; }
        }
        total_walls
    }
}