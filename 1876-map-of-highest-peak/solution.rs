use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (is_water.len(), is_water[0].len());
        let mut height = vec![vec![-1i32; n]; m];
        let mut queue = VecDeque::new();
        for i in 0..m { for j in 0..n {
            if is_water[i][j] == 1 { height[i][j] = 0; queue.push_back((i, j)); }
        }}
        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in [(!0, 0), (1, 0), (0, !0usize), (0, 1usize)] {
                let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if nr < m && nc < n && height[nr][nc] == -1 {
                    height[nr][nc] = height[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
        height
    }
}
