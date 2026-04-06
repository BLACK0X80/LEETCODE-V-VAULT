use std::collections::BTreeMap;

impl Solution {
    pub fn recover_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let n = nums.len();

        for i in 1..n {
            let diff = nums[i] - nums[0];
            if diff == 0 || diff % 2 != 0 { continue; }
            let k = diff / 2;
            let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
            for &x in &nums { *cnt.entry(x).or_insert(0) += 1; }
            let mut res = Vec::new();
            let mut valid = true;
            for x in nums.clone() {
                if *cnt.get(&x).unwrap_or(&0) == 0 { continue; }
                *cnt.get_mut(&x).unwrap() -= 1;
                let hi = x + 2 * k;
                if *cnt.get(&hi).unwrap_or(&0) == 0 { valid = false; break; }
                *cnt.get_mut(&hi).unwrap() -= 1;
                res.push(x + k);
            }
            if valid { return res; }
        }
        vec![]
    }
}
