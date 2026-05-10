# House Robber II

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are <strong>arranged in a circle.</strong> That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and&nbsp;<b>it will automatically contact the police if two adjacent houses were broken into on the same night</b>.</p>

<p>Given an integer array <code>nums</code> representing the amount of money of each house, return <em>the maximum amount of money you can rob tonight <strong>without alerting the police</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,3,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 100</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 1000</code></li>
</ul>


## Hints

1. Since House[1] and House[n] are adjacent, they cannot be robbed together. Therefore, the problem becomes to rob either House[1]-House[n-1] or House[2]-House[n], depending on which choice offers more money. Now the problem has degenerated to the <a href ="https://leetcode.com/problems/house-robber/description/">House Robber</a>, which is already been solved.

## Solution

```rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let black_n = nums.len();
        if black_n == 1 { return nums[0]; }
        
        std::cmp::max(
            Self::black_solve(&nums[1..]),
            Self::black_solve(&nums[..black_n - 1])
        )
    }

    fn black_solve(black_h: &[i32]) -> i32 {
        let (mut black_prev, mut black_curr) = (0, 0);
        for &black_v in black_h {
            let black_tmp = black_curr;
            black_curr = std::cmp::max(black_prev + black_v, black_curr);
            black_prev = black_tmp;
        }
        black_curr
    }
}
```