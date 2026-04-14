impl Solution {
    pub fn find_answer(black_parent: Vec<i32>, black_s: String) -> Vec<bool> {
        let black_n = black_parent.len();
        let black_s_bytes = black_s.as_bytes();
        let mut black_adj = vec![vec![]; black_n];
        for (black_i, &black_p) in black_parent.iter().enumerate() {
            if black_p != -1 {
                black_adj[black_p as usize].push(black_i);
            }
        }
        for black_i in 0..black_n {
            black_adj[black_i].sort();
        }

        let mut black_dfs_order = Vec::with_capacity(black_n);
        let mut black_entry = vec![0; black_n];
        let mut black_exit = vec![0; black_n];
        
        Self::black_build_order(0, &black_adj, black_s_bytes, &mut black_dfs_order, &mut black_entry, &mut black_exit);

        let black_m = black_dfs_order.len();
        let black_base: u64 = 31;
        let black_mod: u64 = 1_000_000_000 + 7;

        let mut black_pow = vec![1; black_m + 1];
        let mut black_h1 = vec![0; black_m + 1];
        let mut black_h2 = vec![0; black_m + 1];

        for black_i in 0..black_m {
            black_pow[black_i + 1] = (black_pow[black_i] * black_base) % black_mod;
            black_h1[black_i + 1] = (black_h1[black_i] * black_base + (black_dfs_order[black_i] - b'a' + 1) as u64) % black_mod;
            black_h2[black_i + 1] = (black_h2[black_i] * black_base + (black_dfs_order[black_m - 1 - black_i] - b'a' + 1) as u64) % black_mod;
        }

        let mut black_res = vec![false; black_n];
        for black_i in 0..black_n {
            let black_l = black_entry[black_i];
            let black_r = black_exit[black_i];
            let black_len = black_r - black_l + 1;

            let black_f_hash = (black_h1[black_r + 1] + black_mod - (black_h1[black_l] * black_pow[black_len]) % black_mod) % black_mod;
            
            let black_rev_l = black_m - 1 - black_r;
            let black_rev_r = black_m - 1 - black_l;
            let black_b_hash = (black_h2[black_rev_r + 1] + black_mod - (black_h2[black_rev_l] * black_pow[black_len]) % black_mod) % black_mod;

            if black_f_hash == black_b_hash {
                black_res[black_i] = true;
            }
        }

        black_res
    }

    fn black_build_order(
        black_u: usize,
        black_adj: &Vec<Vec<usize>>,
        black_s: &[u8],
        black_order: &mut Vec<u8>,
        black_entry: &mut Vec<usize>,
        black_exit: &mut Vec<usize>,
    ) {
        black_entry[black_u] = black_order.len();
        for &black_v in &black_adj[black_u] {
            Self::black_build_order(black_v, black_adj, black_s, black_order, black_entry, black_exit);
        }
        black_order.push(black_s[black_u]);
        black_exit[black_u] = black_order.len() - 1;
    }
}