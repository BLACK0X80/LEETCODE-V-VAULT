impl Solution {
    pub fn can_traverse_all_pairs(black1: Vec<i32>) -> bool {
        let black2 = black1.len();
        if black2 == 1 { return true; }
        
        let mut black3: Vec<usize> = (0..black2).collect();
        let black4 = *black1.iter().max().unwrap() as usize;
        let mut black5 = vec![-1i32; black4 + 1];

        for i in 0..black2 {
            if black1[i] == 1 { return false; }
            
            let mut black6 = black1[i];
            let mut black7 = 2;
            while black7 * black7 <= black6 {
                if black6 % black7 == 0 {
                    if black5[black7 as usize] != -1 {
                        Self::black_union(&mut black3, i, black5[black7 as usize] as usize);
                    } else {
                        black5[black7 as usize] = i as i32;
                    }
                    while black6 % black7 == 0 { black6 /= black7; }
                }
                black7 += 1;
            }
            if black6 > 1 {
                if black5[black6 as usize] != -1 {
                    Self::black_union(&mut black3, i, black5[black6 as usize] as usize);
                } else {
                    black5[black6 as usize] = i as i32;
                }
            }
        }

        let black8 = Self::black_find(&mut black3, 0);
        for i in 1..black2 {
            if Self::black_find(&mut black3, i) != black8 {
                return false;
            }
        }
        true
    }

    fn black_find(black9: &mut Vec<usize>, black10: usize) -> usize {
        if black9[black10] == black10 {
            black10
        } else {
            black9[black10] = Self::black_find(black9, black9[black10]);
            black9[black10]
        }
    }

    fn black_union(black11: &mut Vec<usize>, black12: usize, black13: usize) {
        let black14 = Self::black_find(black11, black12);
        let black15 = Self::black_find(black11, black13);
        if black14 != black15 {
            black11[black14] = black15;
        }
    }
}