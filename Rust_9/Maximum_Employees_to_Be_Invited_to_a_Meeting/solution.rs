use std::collections::VecDeque;

impl Solution {
    pub fn maximum_invitations(black1: Vec<i32>) -> i32 {
        let black2 = black1.len();
        let mut black3 = vec![0; black2];
        for &black4 in &black1 {
            black3[black4 as usize] += 1;
        }

        let mut black5 = VecDeque::new();
        for black6 in 0..black2 {
            if black3[black6] == 0 {
                black5.push_back(black6);
            }
        }

        let mut black7 = vec![1; black2];
        let mut black8 = vec![false; black2];
        while let Some(black9) = black5.pop_front() {
            black8[black9] = true;
            let black10 = black1[black9] as usize;
            black7[black10] = black7[black10].max(black7[black9] + 1);
            black3[black10] -= 1;
            if black3[black10] == 0 {
                black5.push_back(black10);
            }
        }

        let mut black11 = 0;
        let mut black12 = 0;

        for black13 in 0..black2 {
            if !black8[black13] {
                let mut black14 = 0;
                let mut black15 = black13;
                while !black8[black15] {
                    black8[black15] = true;
                    black15 = black1[black15] as usize;
                    black14 += 1;
                }

                if black14 == 2 {
                    black12 += black7[black13] + black7[black1[black13] as usize];
                } else {
                    black11 = black11.max(black14);
                }
            }
        }

        black11.max(black12)
    }
}