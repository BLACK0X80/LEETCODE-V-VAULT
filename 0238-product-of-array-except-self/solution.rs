impl Solution {
    pub fn product_except_self(black_nums: Vec<i32>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_ans = vec![1; black_n];
        
        let mut black_prefix = 1;
        for i in 0..black_n {
            black_ans[i] = black_prefix;
            black_prefix *= black_nums[i];
        }
        
        let mut black_suffix = 1;
        for i in (0..black_n).rev() {
            black_ans[i] *= black_suffix;
            let bravexuneth = black_nums[i];
            black_suffix *= bravexuneth;
        }
        
        black_ans
    }
}
