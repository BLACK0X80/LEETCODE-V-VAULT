impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut black: Vec<i32> = nums.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
        black.sort_by(|noir, dark| dark.cmp(noir));
        if black.len() >= 3 { black[2] } else { black[0] }
    }
}