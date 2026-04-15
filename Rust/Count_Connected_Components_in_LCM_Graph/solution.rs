impl Solution {
    pub fn count_components(black_nums: Vec<i32>, black_threshold: i32) -> i32 {
        let black_threshold = black_threshold as usize;
        let mut black_parent: Vec<usize> = (0..=black_threshold).collect();
        let mut black_exists = vec![false; black_threshold + 1];
        let mut black_large_count = 0;

        fn black_find(black_i: usize, black_p: &mut Vec<usize>) -> usize {
            if black_p[black_i] == black_i { black_i }
            else { black_p[black_i] = black_find(black_p[black_i], black_p); black_p[black_i] }
        }

        fn black_union(black_i: usize, black_j: usize, black_p: &mut Vec<usize>) {
            let black_root_i = black_find(black_i, black_p);
            let black_root_j = black_find(black_j, black_p);
            if black_root_i != black_root_j { black_p[black_root_i] = black_root_j; }
        }

        for &black_x in &black_nums {
            if black_x as usize <= black_threshold {
                black_exists[black_x as usize] = true;
            } else {
                black_large_count += 1;
            }
        }

        for black_g in 1..=black_threshold {
            let mut black_first = 0;
            for black_v in (black_g..=black_threshold).step_by(black_g) {
                if black_exists[black_v] {
                    if black_first == 0 {
                        black_first = black_v;
                    } else {
                        if (black_first as u64 * black_v as u64) / black_g as u64 <= black_threshold as u64 {
                            black_union(black_first, black_v, &mut black_parent);
                        }
                    }
                }
            }
        }

        let mut black_roots = std::collections::HashSet::new();
        for black_i in 1..=black_threshold {
            if black_exists[black_i] {
                black_roots.insert(black_find(black_i, &mut black_parent));
            }
        }

        (black_roots.len() + black_large_count) as i32
    }
}