use std::collections::{VecDeque, BTreeSet};

impl Solution {
    pub fn minimum_visited_cells(black_grid: Vec<Vec<i32>>) -> i32 {
        let (black_m, black_n) = (black_grid.len(), black_grid[0].len());
        let mut black_r: Vec<BTreeSet<usize>> = (0..black_m).map(|_| (0..black_n).collect()).collect();
        let mut black_c: Vec<BTreeSet<usize>> = (0..black_n).map(|_| (0..black_m).collect()).collect();
        let mut black_q = VecDeque::from([(0, 0, 1)]);

        black_r[0].remove(&0);
        black_c[0].remove(&0);

        while let Some((black_i, black_j, black_dist)) = black_q.pop_front() {
            if black_i == black_m - 1 && black_j == black_n - 1 { return black_dist; }
            
            let black_val = black_grid[black_i][black_j] as usize;
            
            let black_row_targets: Vec<_> = black_r[black_i].range((black_j + 1)..(black_j + black_val + 1).min(black_n)).cloned().collect();
            for black_nj in black_row_targets {
                black_q.push_back((black_i, black_nj, black_dist + 1));
                black_r[black_i].remove(&black_nj);
                black_c[black_nj].remove(&black_i);
            }

            let black_col_targets: Vec<_> = black_c[black_j].range((black_i + 1)..(black_i + black_val + 1).min(black_m)).cloned().collect();
            for black_ni in black_col_targets {
                black_q.push_back((black_ni, black_j, black_dist + 1));
                black_r[black_ni].remove(&black_j);
                black_c[black_j].remove(&black_ni);
            }
        }
        -1
    }
}
