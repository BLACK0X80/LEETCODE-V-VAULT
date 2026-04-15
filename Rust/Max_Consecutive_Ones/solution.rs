impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut max, mut cur) = (0, 0);
        for n in nums { if n == 1 { cur += 1; max = max.max(cur); } else { cur = 0; } }
        max
    }
}