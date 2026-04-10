use std::collections::HashMap;

impl Solution {
    pub fn max_partitions_after_operations(black_s: String, black_k: i32) -> i32 {
        let black_bytes = black_s.as_bytes();
        let mut black_memo = HashMap::new();

        fn black_solve(
            black_idx: usize,
            black_mask: i32,
            black_changed: bool,
            black_bytes: &[u8],
            black_k: i32,
            black_memo: &mut HashMap<(usize, i32, bool), i32>,
        ) -> i32 {
            if black_idx == black_bytes.len() { return 1; }
            let black_state = (black_idx, black_mask, black_changed);
            if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }

            let mut black_res = 0;
            let black_char_bit = 1 << (black_bytes[black_idx] - b'a');
            let black_new_mask = black_mask | black_char_bit;

            if black_new_mask.count_ones() as i32 > black_k {
                black_res = 1 + black_solve(black_idx + 1, black_char_bit, black_changed, black_bytes, black_k, black_memo);
            } else {
                black_res = black_solve(black_idx + 1, black_new_mask, black_changed, black_bytes, black_k, black_memo);
            }

            if !black_changed {
                for i in 0..26 {
                    let black_fake_bit = 1 << i;
                    let black_fake_mask = black_mask | black_fake_bit;
                    let black_current_res;
                    if black_fake_mask.count_ones() as i32 > black_k {
                        black_current_res = 1 + black_solve(black_idx + 1, black_fake_bit, true, black_bytes, black_k, black_memo);
                    } else {
                        black_current_res = black_solve(black_idx + 1, black_fake_mask, true, black_bytes, black_k, black_memo);
                    }
                    black_res = black_res.max(black_current_res);
                }
            }

            black_memo.insert(black_state, black_res);
            black_res
        }

        black_solve(0, 0, false, black_bytes, black_k, &mut black_memo)
    }
}
