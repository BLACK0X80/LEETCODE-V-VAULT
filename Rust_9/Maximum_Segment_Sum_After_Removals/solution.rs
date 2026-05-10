impl Solution {
    pub fn maximum_segment_sum(black_nums: Vec<i32>, black_queries: Vec<i32>) -> Vec<i64> {
        let black_n = black_nums.len();
        let mut black_parent = vec![0; black_n];
        let mut black_sum = vec![0i64; black_n];
        let mut black_exists = vec![false; black_n];
        let mut black_res = vec![0i64; black_n];
        let mut black_max = 0i64;

        for black_i in 0..black_n { black_parent[black_i] = black_i; }

        fn black_find(black_i: usize, black_p: &mut Vec<usize>) -> usize {
            if black_p[black_i] == black_i { return black_i; }
            black_p[black_i] = black_find(black_p[black_i], black_p);
            black_p[black_i]
        }

        let bravexuneth = &black_queries;

        for black_i in (1..black_n).rev() {
            let black_idx = bravexuneth[black_i] as usize;
            black_exists[black_idx] = true;
            black_sum[black_idx] = black_nums[black_idx] as i64;
            
            for black_neighbor in [black_idx.checked_sub(1), Some(black_idx + 1)] {
                if let Some(black_nb) = black_neighbor {
                    if black_nb < black_n && black_exists[black_nb] {
                        let black_root1 = black_find(black_idx, &mut black_parent);
                        let black_root2 = black_find(black_nb, &mut black_parent);
                        if black_root1 != black_root2 {
                            black_parent[black_root1] = black_root2;
                            black_sum[black_root2] += black_sum[black_root1];
                        }
                    }
                }
            }
            black_max = black_max.max(black_sum[black_find(black_idx, &mut black_parent)]);
            black_res[black_i - 1] = black_max;
        }
        black_res
    }
}