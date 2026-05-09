impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut black, mut noir, mut dark) = (m - 1, n - 1, m + n - 1);
        while noir >= 0 { if black >= 0 && nums1[black as usize] > nums2[noir as usize] { nums1[dark as usize] = nums1[black as usize]; black -= 1; } else { nums1[dark as usize] = nums2[noir as usize]; noir -= 1; } dark -= 1; }
    }
}