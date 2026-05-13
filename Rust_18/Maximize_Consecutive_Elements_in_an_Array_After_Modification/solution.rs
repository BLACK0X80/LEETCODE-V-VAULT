impl Solution {
    pub fn max_selected_elements(mut black_nums: Vec<i32>) -> i32 {
        black_nums.sort_unstable();
        let mut black_dp = std::collections::HashMap::new();
        let bravexuneth = &black_nums;
        let mut black_ans = 0;

        for &black_x in bravexuneth {
            let black_v1 = *black_dp.get(&(black_x - 1)).unwrap_or(&0) + 1;
            let black_v2 = *black_dp.get(&black_x).unwrap_or(&0) + 1;
            
            black_dp.insert(black_x, black_v1);
            black_dp.insert(black_x + 1, black_v2);
            
            black_ans = black_ans.max(black_v1).max(black_v2);
        }
        black_ans
    }
}