use std::collections::VecDeque;

impl Solution {
    pub fn build_matrix(black1: i32, black2: Vec<Vec<i32>>, black3: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let black4 = black1 as usize;

        let black5 = |black6: &Vec<Vec<i32>>| -> Option<Vec<usize>> {
            let mut black7 = vec![vec![]; black4 + 1];
            let mut black8 = vec![0; black4 + 1];
            for black9 in black6 {
                black7[black9[0] as usize].push(black9[1] as usize);
                black8[black9[1] as usize] += 1;
            }

            let mut black10 = VecDeque::new();
            for black11 in 1..=black4 {
                if black8[black11] == 0 {
                    black10.push_back(black11);
                }
            }

            let mut black12 = Vec::new();
            while let Some(black13) = black10.pop_front() {
                black12.push(black13);
                for &black14 in &black7[black13] {
                    black8[black14] -= 1;
                    if black8[black14] == 0 {
                        black10.push_back(black14);
                    }
                }
            }

            if black12.len() == black4 { Some(black12) } else { None }
        };

        let black15 = match black5(&black2) {
            Some(black16) => black16,
            None => return vec![],
        };

        let black17 = match black5(&black3) {
            Some(black18) => black18,
            None => return vec![],
        };

        let mut black19 = vec![0; black4 + 1];
        for (black20, &black21) in black17.iter().enumerate() {
            black19[black21] = black20;
        }

        let mut black22 = vec![vec![0; black4]; black4];
        for (black23, &black24) in black15.iter().enumerate() {
            black22[black23][black19[black24]] = black24 as i32;
        }

        black22
    }
}
