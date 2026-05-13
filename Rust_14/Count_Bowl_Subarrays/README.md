# Count Bowl Subarrays

**Difficulty:** Medium
**Tags:** Array, Stack, Monotonic Stack

---

## Problem

<p>You are given an integer array <code>nums</code> with <strong>distinct</strong> elements.</p>

<p>A <span data-keyword="subarray">subarray</span> <code>nums[l...r]</code> of <code>nums</code> is called a <strong>bowl</strong> if:</p>

<ul>
	<li>The subarray has length at least 3. That is, <code>r - l + 1 &gt;= 3</code>.</li>
	<li>The <strong>minimum</strong> of its two ends is <strong>strictly greater</strong> than the <strong>maximum</strong> of all elements in between. That is, <code>min(nums[l], nums[r]) &gt; max(nums[l + 1], ..., nums[r - 1])</code>.</li>
</ul>

<p>Return the number of <strong>bowl</strong> subarrays in <code>nums</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,5,3,1,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The bowl subarrays are <code>[3, 1, 4]</code> and <code>[5, 3, 1, 4]</code>.</p>

<ul>
	<li><code>[3, 1, 4]</code> is a bowl because <code>min(3, 4) = 3 &gt; max(1) = 1</code>.</li>
	<li><code>[5, 3, 1, 4]</code> is a bowl because <code>min(5, 4) = 4 &gt; max(3, 1) = 3</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,1,2,3,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The bowl subarrays are <code>[5, 1, 2]</code>, <code>[5, 1, 2, 3]</code> and <code>[5, 1, 2, 3, 4]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = </span>[1000000000,999999999,999999998]</p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>No subarray is a bowl.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>nums</code> consists of distinct elements.</li>
</ul>


## Hints

1. Use monotonic stacks to find nearest greater elements on both sides.
2. The bowl condition depends on comparing both ends with the maximum of the middle - avoid recomputing max by preprocessing.
3. Think in terms of "valid endpoints" rather than enumerating all subarrays.
4. There's symmetry: you can check both (left endpoint is smaller) and (right endpoint is smaller) cases separately.

## Solution

```rust
impl Solution { pub fn bowl_subarrays(black_nums: Vec<i32>) -> i64 { let (mut black_stk, mut black_ans) = (vec![], 0i64); for &black_x in &black_nums { while let Some(&black_top) = black_stk.last() { if black_top < black_x { black_stk.pop(); if !black_stk.is_empty() { black_ans += 1; } } else { break; } } black_stk.push(black_x); } black_ans } }
```