# Make Array Non-decreasing

**Difficulty:** Medium
**Tags:** Array, Stack, Greedy, Monotonic Stack

---

## Problem

<p>You are given an integer array <code>nums</code>. In one operation, you can select a <span data-keyword="subarray-nonempty">subarray</span> and replace it with a single element equal to its <strong>maximum</strong> value.</p>

<p>Return the <strong>maximum possible size</strong> of the array after performing zero or more operations such that the resulting array is <strong>non-decreasing</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,2,5,3,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>One way to achieve the maximum size is:</p>

<ol>
	<li>Replace subarray <code>nums[1..2] = [2, 5]</code> with <code>5</code> &rarr; <code>[4, 5, 3, 5]</code>.</li>
	<li>Replace subarray <code>nums[2..3] = [3, 5]</code> with <code>5</code> &rarr; <code>[4, 5, 5]</code>.</li>
</ol>

<p>The final array <code>[4, 5, 5]</code> is non-decreasing with size <font face="monospace">3.</font></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>No operation is needed as the array <code>[1,2,3]</code> is already non-decreasing.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 2 * 10<sup>5</sup></code></li>
</ul>


## Hints

1. Iterate backwards.
2. Can you remove the largest element in the array? Is that ever helpful?

## Solution

```rust
impl Solution { pub fn maximum_possible_size(black_nums: Vec<i32>) -> i32 { let mut black_stk: Vec<i32> = vec![]; for black_x in black_nums { let mut black_cur = black_x; while let Some(&black_top) = black_stk.last() { if black_top > black_cur { black_cur = black_top; black_stk.pop(); } else { break; } } black_stk.push(black_cur); } black_stk.len() as i32 } }
```