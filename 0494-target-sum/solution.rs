impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = std::collections::HashMap::new();
        dp.insert(0i32, 1i32);
        for n in nums {
            let mut next = std::collections::HashMap::new();
            for (sum, cnt) in dp {
                *next.entry(sum + n).or_insert(0) += cnt;
                *next.entry(sum - n).or_insert(0) += cnt;
            }
            dp = next;
        }
        *dp.get(&target).unwrap_or(&0)
    }
}
