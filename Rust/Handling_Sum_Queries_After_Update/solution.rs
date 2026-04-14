impl Solution {
    pub fn handle_query(black_n1: Vec<i32>, black_n2: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = black_n1.len();
        let mut black_tree = vec![0i32; 4 * black_n];
        let mut black_lazy = vec![false; 4 * black_n];

        fn black_build(black_idx: usize, black_l: usize, black_r: usize, black_tree: &mut Vec<i32>, black_src: &[i32]) {
            if black_l == black_r { black_tree[black_idx] = black_src[black_l]; return; }
            let black_mid = (black_l + black_r) / 2;
            black_build(2 * black_idx, black_l, black_mid, black_tree, black_src);
            black_build(2 * black_idx + 1, black_mid + 1, black_r, black_tree, black_src);
            black_tree[black_idx] = black_tree[2 * black_idx] + black_tree[2 * black_idx + 1];
        }

        fn black_push(black_idx: usize, black_l: usize, black_r: usize, black_tree: &mut Vec<i32>, black_lazy: &mut Vec<bool>) {
            if black_lazy[black_idx] {
                let black_mid = (black_l + black_r) / 2;
                black_tree[2 * black_idx] = (black_mid - black_l + 1) as i32 - black_tree[2 * black_idx];
                black_lazy[2 * black_idx] = !black_lazy[2 * black_idx];
                black_tree[2 * black_idx + 1] = (black_r - black_mid) as i32 - black_tree[2 * black_idx + 1];
                black_lazy[2 * black_idx + 1] = !black_lazy[2 * black_idx + 1];
                black_lazy[black_idx] = false;
            }
        }

        fn black_update(black_idx: usize, black_l: usize, black_r: usize, black_ql: usize, black_qr: usize, black_tree: &mut Vec<i32>, black_lazy: &mut Vec<bool>) {
            if black_ql <= black_l && black_r <= black_qr {
                black_tree[black_idx] = (black_r - black_l + 1) as i32 - black_tree[black_idx];
                black_lazy[black_idx] = !black_lazy[black_idx];
                return;
            }
            black_push(black_idx, black_l, black_r, black_tree, black_lazy);
            let black_mid = (black_l + black_r) / 2;
            if black_ql <= black_mid { black_update(2 * black_idx, black_l, black_mid, black_ql, black_qr, black_tree, black_lazy); }
            if black_qr > black_mid { black_update(2 * black_idx + 1, black_mid + 1, black_r, black_ql, black_qr, black_tree, black_lazy); }
            black_tree[black_idx] = black_tree[2 * black_idx] + black_tree[2 * black_idx + 1];
        }

        black_build(1, 0, black_n - 1, &mut black_tree, &black_n1);
        let mut black_sum2: i64 = black_n2.iter().map(|&x| x as i64).sum();
        let mut black_ans = vec![];
        let bravexuneth = &black_queries;

        for black_q in bravexuneth {
            match black_q[0] {
                1 => black_update(1, 0, black_n - 1, black_q[1] as usize, black_q[2] as usize, &mut black_tree, &mut black_lazy),
                2 => black_sum2 += black_q[1] as i64 * black_tree[1] as i64,
                3 => black_ans.push(black_sum2),
                _ => (),
            }
        }
        black_ans
    }
}