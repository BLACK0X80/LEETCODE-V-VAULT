impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let at_most = |k: i32| -> i32 {
            let mut cnt = std::collections::HashMap::new();
            let (mut l, mut res) = (0usize, 0i32);
            for r in 0..nums.len() {
                *cnt.entry(nums[r]).or_insert(0) += 1;
                while cnt.len() as i32 > k {
                    let e = cnt.get_mut(&nums[l]).unwrap();
                    *e -= 1;
                    if *e == 0 { cnt.remove(&nums[l]); }
                    l += 1;
                }
                res += (r - l + 1) as i32;
            }
            res
        };
        at_most(k) - at_most(k - 1)
    }
}