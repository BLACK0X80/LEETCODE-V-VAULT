impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let black_n = word1.len();
        let black_arr1 = word1.as_bytes();
        let black_arr2 = word2.as_bytes();
        let mut black_dp = vec![vec![None; black_n]; black_n];

        Self::solve(0, 0, black_arr1, black_arr2, black_n, &mut black_dp)
    }

    fn solve(black_i: usize, black_j: usize, black_arr1: &[u8], black_arr2: &[u8], black_n: usize, black_dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if black_i >= black_n { return 0; }
        if black_j >= black_n { return 100000; }

        if let Some(black_val) = black_dp[black_i][black_j] {
            return black_val;
        }

        let black_dont_start = Self::solve(black_i, black_j + 1, black_arr1, black_arr2, black_n, black_dp);
        
        let black_cost = Self::min_opr(black_arr1, black_arr2, black_i, black_j, false)
            .min(Self::min_opr(black_arr1, black_arr2, black_i, black_j, true));
            
        let black_start = black_cost + Self::solve(black_j + 1, black_j + 1, black_arr1, black_arr2, black_n, black_dp);
        
        let black_res = black_dont_start.min(black_start);
        black_dp[black_i][black_j] = Some(black_res);
        black_res
    }

    fn min_opr(black_arr1: &[u8], black_arr2: &[u8], black_i: usize, black_j: usize, black_reversed: bool) -> i32 {
        let mut black_operations = if black_reversed { 1 } else { 0 };
        let mut black_freq = [[0i32; 26]; 26];
        let black_len = black_j - black_i + 1;

        for black_k in 0..black_len {
            let black_idx1 = black_i + black_k;
            let black_idx2 = if black_reversed { black_j - black_k } else { black_i + black_k };

            if black_arr1[black_idx1] != black_arr2[black_idx2] {
                let black_wanted = (black_arr1[black_idx1] - b'a') as usize;
                let black_got = (black_arr2[black_idx2] - b'a') as usize;

                if black_freq[black_got][black_wanted] > 0 {
                    black_freq[black_got][black_wanted] -= 1;
                } else {
                    black_freq[black_wanted][black_got] += 1;
                    black_operations += 1;
                }
            }
        }
        black_operations
    }
}
