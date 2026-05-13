impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let black_rows = target_grid.len();
        let black_cols = target_grid[0].len();
        let mut black_bounds = vec![(61, -1, 61, -1); 61];
        let mut black_exists = vec![false; 61];

        for r in 0..black_rows {
            for c in 0..black_cols {
                let black_color = target_grid[r][c] as usize;
                black_exists[black_color] = true;
                black_bounds[black_color].0 = black_bounds[black_color].0.min(r as i32);
                black_bounds[black_color].1 = black_bounds[black_color].1.max(r as i32);
                black_bounds[black_color].2 = black_bounds[black_color].2.min(c as i32);
                black_bounds[black_color].3 = black_bounds[black_color].3.max(c as i32);
            }
        }

        let mut black_adj = vec![std::collections::HashSet::new(); 61];
        for i in 1..=60 {
            if !black_exists[i] { continue; }
            let (black_r1, black_r2, black_c1, black_c2) = black_bounds[i];
            for r in black_r1..=black_r2 {
                for c in black_c1..=black_c2 {
                    let black_j = target_grid[r as usize][c as usize] as usize;
                    if black_j != i {
                        black_adj[i].insert(black_j);
                    }
                }
            }
        }

        let mut black_state = vec![0; 61];
        for i in 1..=60 {
            if black_exists[i] && black_state[i] == 0 {
                if Self::black_has_cycle(i, &black_adj, &mut black_state) {
                    return false;
                }
            }
        }
        true
    }

    fn black_has_cycle(black_u: usize, black_adj: &Vec<std::collections::HashSet<usize>>, black_state: &mut Vec<i32>) -> bool {
        black_state[black_u] = 1;
        for &black_v in &black_adj[black_u] {
            if black_state[black_v] == 1 { return true; }
            if black_state[black_v] == 0 && Self::black_has_cycle(black_v, black_adj, black_state) { return true; }
        }
        black_state[black_u] = 2;
        false
    }
}