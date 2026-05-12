# Jump Game IX

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>From any index <code>i</code>, you can jump to another index <code>j</code> under the following rules:</p>

<ul>
	<li>Jump to index <code>j</code> where <code>j &gt; i</code> is allowed only if <code>nums[j] &lt; nums[i]</code>.</li>
	<li>Jump to index <code>j</code> where <code>j &lt; i</code> is allowed only if <code>nums[j] &gt; nums[i]</code>.</li>
</ul>

<p>For each index <code>i</code>, find the <strong>maximum</strong> <strong>value</strong> in <code>nums</code> that can be reached by following <strong>any</strong> sequence of valid jumps starting at <code>i</code>.</p>

<p>Return an array <code>ans</code> where <code>ans[i]</code> is the <strong>maximum</strong> <strong>value</strong> reachable starting from index <code>i</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,2,3]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For <code>i = 0</code>: No jump increases the value.</li>
	<li>For <code>i = 1</code>: Jump to <code>j = 0</code> as <code>nums[j] = 2</code> is greater than <code>nums[i]</code>.</li>
	<li>For <code>i = 2</code>: Since <code>nums[2] = 3</code> is the maximum value in <code>nums</code>, no jump increases the value.</li>
</ul>

<p>Thus, <code>ans = [2, 2, 3]</code>.</p>

<ul>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,3,3]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For <code>i = 0</code>: Jump forward to <code>j = 2</code> as <code>nums[j] = 1</code> is less than <code>nums[i] = 2</code>, then from <code>i = 2</code> jump to <code>j = 1</code> as <code>nums[j] = 3</code> is greater than <code>nums[2]</code>.</li>
	<li>For <code>i = 1</code>: Since <code>nums[1] = 3</code> is the maximum value in <code>nums</code>, no jump increases the value.</li>
	<li>For <code>i = 2</code>: Jump to <code>j = 1</code> as <code>nums[j] = 3</code> is greater than <code>nums[2] = 1</code>.</li>
</ul>

<p>Thus, <code>ans = [3, 3, 3]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup>​​​​​​​</code></li>
</ul>


## Hints

1. Think of the array as a directed graph where edges represent valid jumps.
2. From index <code>i</code>, forward jumps go only to smaller values; backward jumps go only to larger values.
3. The maximum reachable value from <code>i</code> is the maximum value in the connected component reachable under these jump rules.
4. You can find connected ranges by looking at prefix maximums and suffix minimums, a cut happens where all values to the left are <= all values to the right.

## Solution

```rust
impl Solution { pub fn max_value(nums: Vec<i32>) -> Vec<i32> { let mut black_ans = vec![0; nums.len()]; let mut black_s: Vec<(i32, usize)> = Vec::new(); for (i, &black_v) in nums.iter().enumerate() { let (mut black_cur_m, mut black_cur_s) = (black_v, i); while black_s.last().map_or(false, |&black_t| black_t.0 > black_v) { let black_t = black_s.pop().unwrap(); black_cur_m = black_cur_m.max(black_t.0); black_cur_s = black_t.1; } black_s.push((black_cur_m, black_cur_s)); } for i in 0..black_s.len() { let black_end = if i + 1 < black_s.len() { black_s[i+1].1 } else { nums.len() }; for j in black_s[i].1..black_end { black_ans[j] = black_s[i].0; } } black_ans } }
```