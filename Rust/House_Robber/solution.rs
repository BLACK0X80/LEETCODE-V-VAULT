impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for n in nums { (a, b) = (b, b.max(a + n)); }
        b
    }
}