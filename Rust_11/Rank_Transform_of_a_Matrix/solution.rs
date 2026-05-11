impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut black_cells: Vec<(i32, usize, usize)> = vec![];
        for i in 0..m { for j in 0..n { black_cells.push((matrix[i][j], i, j)); } }
        black_cells.sort_unstable();
        let mut black_rank = vec![0i32; m + n];
        let mut black_ans = vec![vec![0i32; n]; m];

        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }

        let mut i = 0;
        while i < black_cells.len() {
            let val = black_cells[i].0;
            let mut j = i;
            while j < black_cells.len() && black_cells[j].0 == val { j += 1; }

            let mut black_dsu: Vec<usize> = (0..m+n).collect();
            for k in i..j {
                let (r, c) = (black_cells[k].1, black_cells[k].2);
                let (pr, pc) = (black_find(&mut black_dsu, r), black_find(&mut black_dsu, m+c));
                if pr != pc { black_dsu[pr] = pc; }
            }

            
            let mut black_max = vec![0i32; m+n];
            for k in i..j {
                let (r, c) = (black_cells[k].1, black_cells[k].2);
                let root = black_find(&mut black_dsu, r);
                black_max[root] = black_max[root].max(black_rank[r]).max(black_rank[m+c]);
            }

            for k in i..j {
                let (r, c) = (black_cells[k].1, black_cells[k].2);
                let root = black_find(&mut black_dsu, r);
                black_ans[r][c] = black_max[root] + 1;
                black_rank[r] = black_ans[r][c];
                black_rank[m+c] = black_ans[r][c];
            }
            i = j;
        }
        black_ans
    }
}