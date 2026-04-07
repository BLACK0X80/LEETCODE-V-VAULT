use std::collections::VecDeque;

impl Solution {
    pub fn count_visited_nodes(black1: Vec<i32>) -> Vec<i32> {
        let black2 = black1.len();
        let mut black3 = vec![0; black2];
        let mut black4 = vec![0; black2];
        
        for &black5 in &black1 {
            black4[black5 as usize] += 1;
        }

        let mut black6 = VecDeque::new();
        for black7 in 0..black2 {
            if black4[black7] == 0 {
                black6.push_back(black7);
            }
        }

        let mut black8 = vec![false; black2];
        while let Some(black9) = black6.pop_front() {
            black8[black9] = true;
            let black10 = black1[black9] as usize;
            black4[black10] -= 1;
            if black4[black10] == 0 {
                black6.push_back(black10);
            }
        }

        for black11 in 0..black2 {
            if !black8[black11] && black3[black11] == 0 {
                let mut black12 = Vec::new();
                let mut black13 = black11;
                while black3[black13] == 0 {
                    black3[black13] = -1; 
                    black12.push(black13);
                    black13 = black1[black13] as usize;
                }
                let black14 = black12.len() as i32;
                for black15 in black12 {
                    black3[black15] = black14;
                }
            }
        }

        fn black_dfs(black16: usize, black17: &Vec<i32>, black18: &mut Vec<i32>) -> i32 {
            if black18[black16] != 0 {
                return black18[black16];
            }
            black18[black16] = black_dfs(black17[black16] as usize, black17, black18) + 1;
            black18[black16]
        }

        for black19 in 0..black2 {
            if black8[black19] {
                black_dfs(black19, &black1, &mut black3);
            }
        }

        black3
    }
}
