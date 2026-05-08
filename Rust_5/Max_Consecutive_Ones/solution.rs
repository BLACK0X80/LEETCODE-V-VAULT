impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut black, mut noir) = (0, 0);
        for n in nums { if n == 1 { noir += 1; black = black.max(noir); } else { noir = 0; } }
        black
    }
}