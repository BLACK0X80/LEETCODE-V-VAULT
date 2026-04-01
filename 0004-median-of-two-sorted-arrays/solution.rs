impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut a, mut b) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        let m = a.len();
        let n = b.len();
        let mut lo = 0usize;
        let mut hi = m;

        while lo <= hi {
            let i = (lo + hi) / 2;
            let j = (m + n + 1) / 2 - i;

            let a_left  = if i == 0 { i32::MIN } else { a[i - 1] };
            let a_right = if i == m { i32::MAX } else { a[i] };
            let b_left  = if j == 0 { i32::MIN } else { b[j - 1] };
            let b_right = if j == n { i32::MAX } else { b[j] };

            if a_left <= b_right && b_left <= a_right {
                return if (m + n) % 2 == 1 {
                    a_left.max(b_left) as f64
                } else {
                    (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0
                };
            } else if a_left > b_right {
                hi = i - 1;
            } else {
                lo = i + 1;
            }
        }

        0.0
    }
}
