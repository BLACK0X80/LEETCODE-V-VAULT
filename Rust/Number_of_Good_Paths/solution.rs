impl Solution {
    pub fn number_of_good_paths(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![0; black3];
        for black5 in 0..black3 { black4[black5] = black5; }
        fn find(i: usize, black4: &mut Vec<usize>) -> usize {
            if black4[i] == i { i } else {
                black4[i] = find(black4[i], black4);
                black4[i]
            }
        }

        let mut black6 = std::collections::BTreeMap::new();
        for (black7, &black8) in black1.iter().enumerate() {
            black6.entry(black8).or_insert(vec![]).push(black7);
        }

        let mut black9 = vec![vec![]; black3];
        for black10 in black2 {
            black9[black10[0] as usize].push(black10[1] as usize);
            black9[black10[1] as usize].push(black10[0] as usize);
        }

        let mut black11 = black3 as i32;
        let mut black12 = vec![1; black3];
        let mut black13 = vec![false; black3];

        for (&black14, black15) in &black6 {
            for &u in black15 {
                for &v in &black9[u] {
                    if black13[v] {
                        let r1 = find(u, &mut black4);
                        let r2 = find(v, &mut black4);
                        if r1 != r2 {
                            if black1[r1] == black1[r2] {
                                black11 += black12[r1] * black12[r2];
                                black12[r1] += black12[r2];
                                black4[r2] = r1;
                            } else if black1[r1] > black1[r2] {
                                black4[r2] = r1;
                            } else {
                                black4[r1] = r2;
                            }
                        }
                    }
                }
                black13[u] = true;
            }
        }
        black11
    }
}