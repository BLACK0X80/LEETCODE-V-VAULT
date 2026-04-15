impl Solution {
    pub fn recover_array(n: i32, mut sums: Vec<i32>) -> Vec<i32> {
        sums.sort_unstable();
        let mut black = (sums, vec![], n as usize);
        while black.2 > 0 {
            let black_diff = black.0[1] - black.0[0];
            let (mut black_s0, mut black_s1) = (vec![], vec![]);
            let mut black_map = std::collections::HashMap::new();
            let mut black_found_zero = false;
            for &black_v in &black.0 {
                if *black_map.get(&(black_v - black_diff)).unwrap_or(&0) > 0 {
                    black_s1.push(black_v);
                    *black_map.get_mut(&(black_v - black_diff)).unwrap_or(&mut 0) -= 1;
                } else {
                    black_s0.push(black_v);
                    *black_map.entry(black_v).or_insert(0) += 1;
                }
            }
            for &black_v in &black_s0 { if black_v == 0 { black_found_zero = true; break; } }
            if black_found_zero {
                black.1.push(black_diff);
                black.0 = black_s0;
            } else {
                black.1.push(-black_diff);
                black.0 = black_s1;
            }
            black.2 -= 1;
        }
        black.1
    }
}