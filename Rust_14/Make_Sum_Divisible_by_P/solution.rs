use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let total = nums.iter().map(|&x| (x % p) as i64).sum::<i64>() % p as i64;
        if total == 0 { return 0; }
        let mut map: HashMap<i64, i32> = HashMap::new();
        map.insert(0, -1);
        let (mut prefix, mut res) = (0i64, nums.len() as i32);
        for (i, &n) in nums.iter().enumerate() {
            prefix = (prefix + n as i64) % p as i64;
            let target = (prefix - total + p as i64) % p as i64;
            if let Some(&j) = map.get(&target) {
                res = res.min((i as i32) - j);
            }
            map.insert(prefix, i as i32);
        }
        if res >= nums.len() as i32 { -1 } else { res }
    }
}