impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut black, mut noir) = (0, nums.len() - 1);
        while black < noir { while black < noir && nums[black] % 2 == 0 { black += 1; } while black < noir && nums[noir] % 2 != 0 { noir -= 1; } if black < noir { nums.swap(black, noir); } }
        nums
    }
}