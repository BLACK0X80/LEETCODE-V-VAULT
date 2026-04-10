impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut black_v = reward_values;
        black_v.sort_unstable();
        black_v.dedup();

        let mut black_bs = vec![0u64; 1570];
        black_bs[0] = 1;

        for v in black_v {
            let v = v as usize;
            let black_words = v / 64;
            let black_bits = v % 64;

            if black_bits == 0 {
                for i in 0..black_words {
                    black_bs[i + black_words] |= black_bs[i];
                }
            } else {
                let black_inv_bits = 64 - black_bits;
                for i in 0..black_words {
                    let black_val = black_bs[i];
                    black_bs[i + black_words] |= black_val << black_bits;
                    black_bs[i + black_words + 1] |= black_val >> black_inv_bits;
                }
                let black_mask = (1u64 << black_bits) - 1;
                let black_val = black_bs[black_words] & black_mask;
                black_bs[black_words * 2] |= black_val << black_bits;
                if black_words * 2 + 1 < 1570 {
                    black_bs[black_words * 2 + 1] |= black_val >> black_inv_bits;
                }
            }
        }

        for i in (0..1570).rev() {
            if black_bs[i] != 0 {
                return (i * 64 + 63 - black_bs[i].leading_zeros() as usize) as i32;
            }
        }
        0
    }
}
