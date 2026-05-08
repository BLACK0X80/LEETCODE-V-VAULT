impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_m = (r - l + 1) as usize;
        let black_dim = 2 * black_m;
        
        let mut black_matrix = vec![vec![0i64; black_dim]; black_dim];
        for black_i in 0..black_m {
            for black_j in 0..black_m {
                if black_i < black_j {
                    black_matrix[black_j + black_m][black_i] = 1;
                } else if black_i > black_j {
                    black_matrix[black_j][black_i + black_m] = 1;
                }
            }
        }

        fn black_mul(black_a: &Vec<Vec<i64>>, black_b: &Vec<Vec<i64>>, black_size: usize, black_q: i64) -> Vec<Vec<i64>> {
            let mut black_c = vec![vec![0i64; black_size]; black_size];
            for black_i in 0..black_size {
                for black_k in 0..black_size {
                    if black_a[black_i][black_k] == 0 { continue; }
                    for black_j in 0..black_size {
                        black_c[black_i][black_j] = (black_c[black_i][black_j] + black_a[black_i][black_k] * black_b[black_k][black_j]) % black_q;
                    }
                }
            }
            black_c
        }

        let mut black_res = vec![vec![0i64; black_dim]; black_dim];
        for black_i in 0..black_dim { black_res[black_i][black_i] = 1; }
        let mut black_base = black_matrix;
        let mut black_exp = n as i64 - 1;
        while black_exp > 0 {
            if black_exp % 2 == 1 { black_res = black_mul(&black_res, &black_base, black_dim, black_mod); }
            black_base = black_mul(&black_base, &black_base, black_dim, black_mod);
            black_exp /= 2;
        }

        let mut black_total = 0i64;
        for black_i in 0..black_m {
            for black_j in 0..black_dim {
                black_total = (black_total + black_res[black_j][black_i] + black_res[black_j][black_i + black_m]) % black_mod;
            }
        }
        (black_total % black_mod) as i32
    }
}