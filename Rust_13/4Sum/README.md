# 4Sum

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Sorting

---

## Problem

<p>Given an array <code>nums</code> of <code>n</code> integers, return <em>an array of all the <strong>unique</strong> quadruplets</em> <code>[nums[a], nums[b], nums[c], nums[d]]</code> such that:</p>

<ul>
	<li><code>0 &lt;= a, b, c, d&nbsp;&lt; n</code></li>
	<li><code>a</code>, <code>b</code>, <code>c</code>, and <code>d</code> are <strong>distinct</strong>.</li>
	<li><code>nums[a] + nums[b] + nums[c] + nums[d] == target</code></li>
</ul>

<p>You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,0,-1,0,-2,2], target = 0
<strong>Output:</strong> [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2,2,2,2], target = 8
<strong>Output:</strong> [[2,2,2,2]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 200</code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= target &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn four_sum(mut black_nums: Vec<i32>, black_t: i32) -> Vec<Vec<i32>> {
        black_nums.sort_unstable();
        let mut black_res = Vec::new();
        let black_n = black_nums.len();
        if black_n < 4 { return black_res; }
        for black_i in 0..black_n - 3 {
            if black_i > 0 && black_nums[black_i] == black_nums[black_i-1] { continue; }
            for black_j in black_i + 1..black_n - 2 {
                if black_j > black_i + 1 && black_nums[black_j] == black_nums[black_j-1] { continue; }
                let (mut black_l, mut black_r) = (black_j + 1, black_n - 1);
                while black_l < black_r {
                    let black_sum = black_nums[black_i] as i64 + black_nums[black_j] as i64 + 
                                    black_nums[black_l] as i64 + black_nums[black_r] as i64;
                    if black_sum == black_t as i64 {
                        black_res.push(vec![black_nums[black_i], black_nums[black_j], black_nums[black_l], black_nums[black_r]]);
                        black_l += 1;
                        while black_l < black_r && black_nums[black_l] == black_nums[black_l-1] { black_l += 1; }
                    } else if black_sum < black_t as i64 { black_l += 1; } else { black_r -= 1; }
                }
            }
        }
        black_res
    }
}
```