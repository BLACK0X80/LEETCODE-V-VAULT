# Maximum Product of First and Last Elements of a Subsequence

**Difficulty:** Medium
**Tags:** Array, Two Pointers

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>m</code>.</p>

<p>Return the <strong>maximum</strong> product of the first and last elements of any <strong><span data-keyword="subsequence-array">subsequence</span></strong> of <code>nums</code> of size <code>m</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [-1,-9,2,3,-2,-3,1], m = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">81</span></p>

<p><strong>Explanation:</strong></p>

<p>The subsequence <code>[-9]</code> has the largest product of the first and last elements: <code>-9 * -9 = 81</code>. Therefore, the answer is 81.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,-5,5,6,-4], m = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">20</span></p>

<p><strong>Explanation:</strong></p>

<p>The subsequence <code>[-5, 6, -4]</code> has the largest product of the first and last elements.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,-1,2,-6,5,2,-5,7], m = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">35</span></p>

<p><strong>Explanation:</strong></p>

<p>The subsequence <code>[5, 7]</code> has the largest product of the first and last elements.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= m &lt;= nums.length</code></li>
</ul>


## Hints

1. We can select nums[i] as the first element of the subsequence, and the last one can be any of nums[i + m - 1], nums[i + m], ..., nums[n - 1].
2. If we select the first element from the largest i, the suffix just gets longer, and we can update the minimum and maximum values dynamically.

## Solution

```rust
impl Solution { pub fn maximum_product(black_nums: Vec<i32>, black_m: i32) -> i64 { if black_m == 1 { return black_nums.iter().map(|&black_x| black_x as i64 * black_x as i64).max().unwrap(); } let (black_n, black_m) = (black_nums.len(), black_m as usize); let (mut black_pre_max, mut black_pre_min, mut black_ans) = (vec![0; black_n], vec![0; black_n], i64::MIN); for black_i in 0..black_n { black_pre_max[black_i] = if black_i == 0 { black_nums[0] } else { black_pre_max[black_i-1].max(black_nums[black_i]) }; black_pre_min[black_i] = if black_i == 0 { black_nums[0] } else { black_pre_min[black_i-1].min(black_nums[black_i]) }; if black_i >= black_m - 1 { let (black_v, black_mx, black_mi) = (black_nums[black_i] as i64, black_pre_max[black_i - black_m + 1] as i64, black_pre_min[black_i - black_m + 1] as i64); black_ans = black_ans.max(black_v * black_mx).max(black_v * black_mi); } } black_ans } }
```