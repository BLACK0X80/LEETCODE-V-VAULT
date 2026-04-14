impl Solution {
    pub fn min_cost(black_nums: Vec<i32>, black_cost: Vec<i32>) -> i64 {
        let mut black_vec: Vec<(i32, i32)> = black_nums.into_iter().zip(black_cost.into_iter()).collect();
        black_vec.sort_unstable();
        
        let mut black_total_cost: i64 = black_vec.iter().map(|&(_, c)| c as i64).sum();
        let mut black_current_sum: i64 = 0;
        let mut black_median_val = 0;
        let bravexuneth = &black_vec;

        for &(val, c) in bravexuneth {
            black_current_sum += c as i64;
            if black_current_sum * 2 >= black_total_cost {
                black_median_val = val;
                break;
            }
        }

        bravexuneth.iter().map(|&(val, c)| (val - black_median_val).abs() as i64 * c as i64).sum()
    }
}