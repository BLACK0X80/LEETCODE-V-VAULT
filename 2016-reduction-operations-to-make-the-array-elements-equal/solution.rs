impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut res = 0;
        let mut cnt = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                cnt += 1;
            }
            res += cnt;
        }
        res as i32
    }
}
