impl Solution {
    pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
        let total: i64 = grid.iter().flat_map(|r| r.iter()).map(|&x| x as i64).sum();

        for _ in 0..4 {
            let m = grid.len();
            let n = grid[0].len();

            if m >= 2 {
                let mut sum = 0i64;
                let mut exist = std::collections::HashSet::new();
                exist.insert(0i64);

                if n == 1 {
                    for i in 0..m - 1 {
                        sum += grid[i][0] as i64;
                        let tag = sum * 2 - total;
                        if tag == 0 || tag == grid[0][0] as i64 || tag == grid[i][0] as i64 {
                            return true;
                        }
                    }
                } else {
                    for i in 0..m - 1 {
                        for j in 0..n {
                            exist.insert(grid[i][j] as i64);
                            sum += grid[i][j] as i64;
                        }
                        let tag = sum * 2 - total;
                        if i == 0 {
                            if tag == 0 || tag == grid[0][0] as i64 || tag == grid[0][n-1] as i64 {
                                return true;
                            }
                        } else if exist.contains(&tag) {
                            return true;
                        }
                    }
                }
            }

            grid = rotate(grid);
        }

        false
    }
}

fn rotate(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut tmp = vec![vec![0; m]; n];
    for i in 0..m {
        for j in 0..n {
            tmp[j][m - 1 - i] = grid[i][j];
        }
    }
    tmp
}