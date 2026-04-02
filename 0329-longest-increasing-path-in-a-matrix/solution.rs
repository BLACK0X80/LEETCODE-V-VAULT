impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut memo = vec![vec![0i32; n]; m];
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                result = result.max(dfs(&matrix, &mut memo, i, j, m, n));
            }
        }
        result
    }
}

fn dfs(matrix: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, r: usize, c: usize, m: usize, n: usize) -> i32 {
    if memo[r][c] != 0 { return memo[r][c]; }
    let dirs = [(0i32,1),(0,-1),(1,0),(-1,0)];
    let mut best = 1;
    for (dr, dc) in dirs {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if matrix[nr][nc] > matrix[r][c] {
                best = best.max(1 + dfs(matrix, memo, nr, nc, m, n));
            }
        }
    }
    memo[r][c] = best;
    best
}
