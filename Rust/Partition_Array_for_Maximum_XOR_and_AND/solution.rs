impl Solution {
    pub fn maximize_xor_and_xor(black_a: Vec<i32>) -> i64 {
        let black_n = black_a.len();
        let black_m = 1 << black_n;
        let mut black_and_mask = vec![0i32; black_m];
        let mut black_xor_mask = vec![0i32; black_m];

        for black_mask in 1..black_m {
            let black_lsb = black_mask.trailing_zeros() as usize;
            let black_prev = black_mask & !(1 << black_lsb);
            black_xor_mask[black_mask] = black_xor_mask[black_prev] ^ black_a[black_lsb];
            black_and_mask[black_mask] = if black_prev != 0 {
                black_and_mask[black_prev] & black_a[black_lsb]
            } else {
                black_a[black_lsb]
            };
        }

        let mut black_answer = 0i64;
        for black_mask_b in 0..black_m {
            let black_and_b = black_and_mask[black_mask_b] as i64;
            let black_mask_s = (black_m - 1) ^ black_mask_b;
            let black_s = black_xor_mask[black_mask_s];

            let mut black_basis = [0i32; 31];
            for black_i in 0..black_n {
                if (black_mask_s & (1 << black_i)) != 0 {
                    let mut black_v = black_a[black_i] & !black_s;
                    for black_bit in (0..=30).rev() {
                        if (black_v & (1 << black_bit)) == 0 {
                            continue;
                        }
                        if black_basis[black_bit] == 0 {
                            black_basis[black_bit] = black_v;
                            break;
                        }
                        black_v ^= black_basis[black_bit];
                    }
                }
            }

            let mut black_mx = 0i32;
            for black_bit in (0..=30).rev() {
                black_mx = black_mx.max(black_mx ^ black_basis[black_bit]);
            }

            let black_current_val = black_and_b + black_s as i64 + 2 * black_mx as i64;
            if black_current_val > black_answer {
                black_answer = black_current_val;
            }
        }

        black_answer
    }
}