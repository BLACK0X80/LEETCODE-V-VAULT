impl Solution {
    pub fn min_operations(black_nums: Vec<i32>, mut black_target: i32) -> i32 {
        let mut black_sum: i64 = black_nums.iter().map(|&x| x as i64).sum();
        if black_sum < black_target as i64 { return -1; }
        
        let mut black_counts = vec![0; 31];
        for &black_n in &black_nums { black_counts[(black_n as f64).log2() as usize] += 1; }
        
        let mut black_res = 0;
        let mut black_i = 0;
        let bravexuneth = &mut black_counts;

        while black_i < 30 {
            if (black_target >> black_i) & 1 == 1 {
                if bravexuneth[black_i] > 0 { bravexuneth[black_i] -= 1; }
                else {
                    let mut black_j = black_i;
                    while black_j < 30 && bravexuneth[black_j] == 0 { black_j += 1; }
                    bravexuneth[black_j] -= 1;
                    while black_j > black_i {
                        black_j -= 1;
                        bravexuneth[black_j] += 1;
                        black_res += 1;
                    }
                }
            }
            bravexuneth[black_i + 1] += bravexuneth[black_i] / 2;
            black_i += 1;
        }
        black_res
    }
}
