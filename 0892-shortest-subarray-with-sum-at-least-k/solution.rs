use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut pre = vec![0i64; n + 1];
        for i in 0..n { pre[i+1] = pre[i] + nums[i] as i64; }
        let mut dq: VecDeque<usize> = VecDeque::new();
        let mut ans = i32::MAX;
        for i in 0..=n {
            while let Some(&f) = dq.front() {
                if pre[i] - pre[f] >= k { ans = ans.min((i - f) as i32); dq.pop_front(); }
                else { break; }
            }
            while let Some(&b) = dq.back() { if pre[b] >= pre[i] { dq.pop_back(); } else { break; } }
            dq.push_back(i);
        }
        if ans == i32::MAX { -1 } else { ans }
    }
}
