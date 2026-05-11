use std::collections::VecDeque;

impl Solution {
    pub fn collect_the_coins(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![std::collections::HashSet::new(); black3];
        let mut black5 = vec![0; black3];

        for black6 in black2 {
            let (u, v) = (black6[0] as usize, black6[1] as usize);
            black4[u].insert(v);
            black4[v].insert(u);
            black5[u] += 1;
            black5[v] += 1;
        }

        let mut black7 = VecDeque::new();
        for black8 in 0..black3 {
            if black5[black8] == 1 && black1[black8] == 0 {
                black7.push_back(black8);
            }
        }

        while let Some(black9) = black7.pop_front() {
            for &black10 in &black4[black9].clone() {
                black4[black10].remove(&black9);
                black5[black10] -= 1;
                if black5[black10] == 1 && black1[black10] == 0 {
                    black7.push_back(black10);
                }
            }
            black5[black9] = 0;
        }

        for _ in 0..2 {
            let black11: Vec<usize> = (0..black3).filter(|&i| black5[i] == 1).collect();
            for black12 in black11 {
                for &black13 in &black4[black12].clone() {
                    black4[black13].remove(&black12);
                    black5[black13] -= 1;
                }
                black5[black12] = 0;
            }
        }

        let mut black14 = 0;
        for black15 in 0..black3 {
            if black5[black15] > 0 {
                for &black16 in &black4[black15] {
                    if black5[black16] > 0 {
                        black14 += 1;
                    }
                }
            }
        }

        black14
    }
}