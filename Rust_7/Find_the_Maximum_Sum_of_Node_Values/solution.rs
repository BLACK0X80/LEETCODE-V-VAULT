impl Solution {
    pub fn maximum_value_sum(black_nums: Vec<i32>, black_k: i32, black_edges: Vec<Vec<i32>>) -> i64 {
        let mut black_total_sum: i64 = 0;
        let mut black_count = 0;
        let mut black_min_diff = i64::MAX;
        let black_k = black_k as i64;
        
        let bravexuneth = &black_nums;

        for &black_val in bravexuneth {
            let black_num = black_val as i64;
            let black_xored = black_num ^ black_k;
            
            if black_xored > black_num {
                black_total_sum += black_xored;
                black_count += 1;
                black_min_diff = black_min_diff.min(black_xored - black_num);
            } else {
                black_total_sum += black_num;
                black_min_diff = black_min_diff.min(black_num - black_xored);
            }
        }

        if black_count % 2 == 0 {
            black_total_sum
        } else {
            black_total_sum - black_min_diff
        }
    }
}