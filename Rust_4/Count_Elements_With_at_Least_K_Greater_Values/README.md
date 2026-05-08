# Count Elements With at Least K Greater Values

**Difficulty:** Medium
**Tags:** Array, Binary Search, Divide and Conquer, Sorting, Quickselect

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and an integer <code>k</code>.</p>

<p>An element in <code>nums</code> is said to be <strong>qualified</strong> if there exist <strong>at least</strong> <code>k</code> elements in the array that are <strong>strictly greater</strong> than it.</p>

<p>Return an integer denoting the total number of qualified elements in <code>nums</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The elements 1 and 2 each have at least <code>k = 1</code> element greater than themselves.<br />
​​​​​​​No element is greater than 3. Therefore, the answer is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,5,5], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>Since all elements are equal to 5, no element is greater than the other. Therefore, the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt; n</code></li>
</ul>


## Hints

1. Sort <code>nums</code>, build distinct <code>values</code> and <code>count</code>.
2. For each <code>val</code>: find <code>upper_bound</code>, compute <code>greater = n - upper_bound_idx</code>; if <code>greater >= k</code> add <code>count[val]</code> to <code>ans</code>.

## Solution

```rust
impl Solution { pub fn count_elements(mut nums: Vec<i32>, k: i32) -> i32 { nums.sort_unstable(); let (mut black_ans, black_n) = (0, nums.len()); for black_i in 0..black_n { let (mut black_l, mut black_r, mut black_j) = (black_i, black_n - 1, black_n); while black_l <= black_r { let black_m = black_l + (black_r - black_l) / 2; if nums[black_m] > nums[black_i] { black_j = black_m; black_r = black_m - 1; } else { black_l = black_m + 1; } } if black_n - black_j >= k as usize { black_ans += 1; } } black_ans } }
```