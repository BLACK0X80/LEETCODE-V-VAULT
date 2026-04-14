impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut mr = 0; let mut mc = 0;
        let mut cr = 0; let mut cc = 0;
        let mut fr = 0; let mut fc = 0;
        let g: Vec<Vec<u8>> = grid.iter().map(|s| s.bytes().collect()).collect();
        for r in 0..rows {
            for c in 0..cols {
                match g[r][c] {
                    b'M' => { mr = r; mc = c; }
                    b'C' => { cr = r; cc = c; }
                    b'F' => { fr = r; fc = c; }
                    _ => {}
                }
            }
        }
        let total = 1000 * rows * rows * cols * cols;
        let mut memo = vec![-1i8; total];
        fn solve(step: usize, mr: usize, mc: usize, cr: usize, cc: usize,
                 cj: i32, mj: i32, rows: usize, cols: usize,
                 g: &[Vec<u8>], fr: usize, fc: usize, memo: &mut [i8]) -> bool {
            if mr == fr && mc == fc { return true; }
            if cr == fr && cc == fc { return false; }
            if mr == cr && mc == cc { return false; }
            if step >= 1000 { return false; }
            let idx = (((step * rows + mr) * cols + mc) * rows + cr) * cols + cc;
            if memo[idx] != -1 { return memo[idx] == 1; }
            let is_mouse = step % 2 == 0;
            let jump = if is_mouse { mj } else { cj };
            let res = if is_mouse {
                let mut win = false;
                for &(dr, dc) in [(0,1), (0,-1), (1,0), (-1,0)].iter() {
                    for d in 0..=jump {
                        let nr = mr as i32 + dr * d;
                        let nc = mc as i32 + dc * d;
                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 || g[nr as usize][nc as usize] == b'#' { break; }
                        if solve(step + 1, nr as usize, nc as usize, cr, cc, cj, mj, rows, cols, g, fr, fc, memo) { win = true; break; }
                    }
                    if win { break; }
                }
                win
            } else {
                let mut lose = true;
                for &(dr, dc) in [(0,1), (0,-1), (1,0), (-1,0)].iter() {
                    for d in 0..=jump {
                        let nr = cr as i32 + dr * d;
                        let nc = cc as i32 + dc * d;
                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 || g[nr as usize][nc as usize] == b'#' { break; }
                        if !solve(step + 1, mr, mc, nr as usize, nc as usize, cj, mj, rows, cols, g, fr, fc, memo) { lose = false; break; }
                    }
                    if !lose { break; }
                }
                lose
            };
            memo[idx] = if res { 1 } else { 0 };
            res
        }
        solve(0, mr, mc, cr, cc, cat_jump, mouse_jump, rows, cols, &g, fr, fc, &mut memo)
    }
}