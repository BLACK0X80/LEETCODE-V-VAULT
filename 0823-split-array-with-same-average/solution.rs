use std::collections::HashSet;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let total: i32 = nums.iter().sum();
        let half = n / 2;

        let mut left_sets = vec![HashSet::new(); half + 1];
        left_sets[0].insert(0);
        for &x in &nums[..half] {
            for k in (0..half).rev() {
                let adds: Vec<i32> = left_sets[k].iter().map(|&s| s + x).collect();
                for v in adds { left_sets[k+1].insert(v); }
            }
        }

        let rn = n - half;
        let mut right_sets = vec![HashSet::new(); rn + 1];
        right_sets[0].insert(0);
        for &x in &nums[half..] {
            for k in (0..rn).rev() {
                let adds: Vec<i32> = right_sets[k].iter().map(|&s| s + x).collect();
                for v in adds { right_sets[k+1].insert(v); }
            }
        }

        for k in 1..n {
            if (total * k as i32) % n as i32 != 0 { continue; }
            let target = total * k as i32 / n as i32;
            for lk in 0..=k.min(half) {
                let rk = k - lk;
                if rk > rn { continue; }
                if (lk == 0 && rk == n) || (lk == half && rk == 0 && k == n) { continue; }
                for &rs in &right_sets[rk] {
                    if left_sets[lk].contains(&(target - rs)) { return true; }
                }
            }
        }
        false
    }
}
