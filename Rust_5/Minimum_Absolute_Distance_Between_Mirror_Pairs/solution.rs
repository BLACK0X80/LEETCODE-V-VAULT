impl Solution {
    pub fn min_mirror_pair_distance(black_nums: Vec<i32>) -> i32 {
        { let (mut black_map, mut black_res) = (std::collections::HashMap::new(), i32::MAX); let black_rev = |mut black_x: i32| { let mut black_r = 0; while black_x > 0 { black_r = black_r * 10 + black_x % 10; black_x /= 10; } black_r }; black_map.insert(black_rev(black_nums[0]), 0); for black_j in 1..black_nums.len() { if let Some(&black_idx) = black_map.get(&black_nums[black_j]) { black_res = black_res.min(black_j as i32 - black_idx); } black_map.insert(black_rev(black_nums[black_j]), black_j as i32); } if black_res == i32::MAX { -1 } else { black_res } }
    }
}