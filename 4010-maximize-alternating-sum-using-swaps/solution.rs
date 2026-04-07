impl Solution {
    pub fn max_alternating_sum(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> i64 {
        let black3 = black1.len();
        let mut black4 = (0..black3).collect::<Vec<_>>();
        fn find(b: &mut Vec<usize>, i: usize) -> usize {
            if b[i] == i { i } else { b[i] = find(b, b[i]); b[i] }
        }
        for s in black2 {
            let r1 = find(&mut black4, s[0] as usize);
            let r2 = find(&mut black4, s[1] as usize);
            if r1 != r2 { black4[r1] = r2; }
        }
        let mut black5 = std::collections::HashMap::new();
        for i in 0..black3 {
            let r = find(&mut black4, i);
            let entry = black5.entry(r).or_insert((Vec::new(), Vec::new()));
            if i % 2 == 0 { entry.0.push(i); } else { entry.1.push(i); }
        }
        let mut black6 = 0i64;
        for (_, (evens, odds)) in black5 {
            let mut vals = Vec::new();
            for &idx in &evens { vals.push(black1[idx]); }
            for &idx in &odds { vals.push(black1[idx]); }
            vals.sort_unstable_by(|a, b| b.cmp(a));
            for i in 0..evens.len() { black6 += vals[i] as i64; }
            for i in evens.len()..vals.len() { black6 -= vals[i] as i64; }
        }
        black6
    }
}
