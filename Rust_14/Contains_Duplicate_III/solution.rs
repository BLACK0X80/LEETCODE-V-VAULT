use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let k = index_diff as usize;
        let t = value_diff as i64;
        let w = t + 1;
        let mut buckets: HashMap<i64, i64> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let n = num as i64;
            let id = if n >= 0 { n / w } else { (n + 1) / w - 1 };

            if buckets.contains_key(&id) { return true; }
            if buckets.get(&(id - 1)).map_or(false, |&v| n - v <= t) { return true; }
            if buckets.get(&(id + 1)).map_or(false, |&v| v - n <= t) { return true; }

            buckets.insert(id, n);
            if i >= k { 
                let old = nums[i - k] as i64;
                let old_id = if old >= 0 { old / w } else { (old + 1) / w - 1 };
                buckets.remove(&old_id);
            }
        }

        false
    }
}