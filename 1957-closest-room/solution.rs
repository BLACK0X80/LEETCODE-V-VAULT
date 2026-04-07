use std::collections::BTreeSet;

impl Solution {
    pub fn closest_room(mut black1: Vec<Vec<i32>>, black2: Vec<Vec<i32>>) -> Vec<i32> {
        black1.sort_unstable_by_key(|r| -r[1]);
        let mut black3: Vec<usize> = (0..black2.len()).collect();
        black3.sort_unstable_by_key(|&i| -black2[i][1]);

        let mut black4 = vec![-1; black2.len()];
        let mut black5 = BTreeSet::new();
        let mut black6 = 0;

        for &idx in &black3 {
            let pref = black2[idx][0];
            let min_s = black2[idx][1];

            while black6 < black1.len() && black1[black6][1] >= min_s {
                black5.insert(black1[black6][0]);
                black6 += 1;
            }

            let mut best = -1;
            let mut min_diff = i32::MAX;

            if let Some(&id) = black5.range(pref..).next() {
                best = id;
                min_diff = id - pref;
            }
            if let Some(&id) = black5.range(..pref).next_back() {
                if pref - id <= min_diff {
                    best = id;
                }
            }
            black4[idx] = best;
        }
        black4
    }
}
