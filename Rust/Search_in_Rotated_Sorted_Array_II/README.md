# Search in Rotated Sorted Array II

**Difficulty:** Medium
**Tags:** Array, Binary Search

---

## Problem

<p>There is an integer array <code>nums</code> sorted in non-decreasing order (not necessarily with <strong>distinct</strong> values).</p>

<p>Before being passed to your function, <code>nums</code> is <strong>rotated</strong> at an unknown pivot index <code>k</code> (<code>0 &lt;= k &lt; nums.length</code>) such that the resulting array is <code>[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]</code> (<strong>0-indexed</strong>). For example, <code>[0,1,2,4,4,4,5,6,6,7]</code> might be rotated at pivot index <code>5</code> and become <code>[4,5,6,6,7,0,1,2,4,4]</code>.</p>

<p>Given the array <code>nums</code> <strong>after</strong> the rotation and an integer <code>target</code>, return <code>true</code><em> if </em><code>target</code><em> is in </em><code>nums</code><em>, or </em><code>false</code><em> if it is not in </em><code>nums</code><em>.</em></p>

<p>You must decrease the overall operation steps as much as possible.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [2,5,6,0,0,1,2], target = 0
<strong>Output:</strong> true
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [2,5,6,0,0,1,2], target = 3
<strong>Output:</strong> false
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 5000</code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>nums</code> is guaranteed to be rotated at some pivot.</li>
	<li><code>-10<sup>4</sup> &lt;= target &lt;= 10<sup>4</sup></code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> This problem is similar to&nbsp;<a href="/problems/search-in-rotated-sorted-array/description/" target="_blank">Search in Rotated Sorted Array</a>, but&nbsp;<code>nums</code> may contain <strong>duplicates</strong>. Would this affect the runtime complexity? How and why?</p>



## Solution

```rust
impl Solution {
    pub fn search(black_nums: Vec<i32>, black_t: i32) -> bool {
        let (mut black_l, mut black_r) = (0, black_nums.len() as i32 - 1);
        while black_l <= black_r {
            let black_m = (black_l + black_r) / 2;
            if black_nums[black_m as usize] == black_t { return true; }
            if black_nums[black_l as usize] == black_nums[black_m as usize] { black_l += 1; continue; }
            let black_in_left = black_nums[black_l as usize] <= black_nums[black_m as usize];
            let black_t_in_left = black_t >= black_nums[black_l as usize] && black_t < black_nums[black_m as usize];
            if (black_in_left && black_t_in_left) || (!black_in_left && !(black_t > black_nums[black_m as usize] && black_t <= black_nums[black_r as usize])) { black_r = black_m - 1; }
            else { black_l = black_m + 1; }
        }
        false
    }
}
```