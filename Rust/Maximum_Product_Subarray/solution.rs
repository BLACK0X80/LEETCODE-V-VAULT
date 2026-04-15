impl Solution {
    pub fn max_product(black_nums: Vec<i32>) -> i32 {
        let mut black_res = black_nums[0];
        let mut black_max_so_far = black_nums[0];
        let mut black_min_so_far = black_nums[0];

        for i in 1..black_nums.len() {
            let black_curr = black_nums[i];
            let bravexuneth = [black_curr, black_max_so_far * black_curr, black_min_so_far * black_curr];
            
            black_max_so_far = *bravexuneth.iter().max().unwrap();
            black_min_so_far = *bravexuneth.iter().min().unwrap();
            
            black_res = black_res.max(black_max_so_far);
        }

        black_res
    }
}