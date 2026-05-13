# Maximize Fixed Points After Deletions

**Difficulty:** Hard
**Tags:** 

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>A position <code>i</code> is called a <strong>fixed point</strong> if <code>nums[i] == i</code>.</p>

<p>You are allowed to delete <strong>any</strong> number of elements (including zero) from the array. After each deletion, the remaining elements <strong>shift left</strong>, and indices are reassigned starting from 0.</p>

<p>Return an integer denoting the <strong>maximum</strong> number of fixed points that can be achieved after performing any number of deletions.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Delete <code>nums[1] = 2</code>. The array becomes <code>[0, 1]</code>.</li>
	<li>Now, <code>nums[0] = 0</code> and <code>nums[1] = 1</code>, so both indices are fixed points.</li>
	<li>Thus, the answer is 2.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Do not delete any elements. The array remains <code>[3, 1, 2]</code>.</li>
	<li>Here, <code>nums[1] = 1</code> and <code>nums[2] = 2</code>, so these indices are fixed points.</li>
	<li>Thus, the answer is 2.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,0,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Delete <code>nums[0] = 1</code>. The array becomes <code>[0, 1, 2]</code>.</li>
	<li>Now, <code>nums[0] = 0</code>, <code>nums[1] = 1</code>, and <code>nums[2] = 2</code>, so all indices are fixed points.</li>
	<li>Thus, the answer is 3.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. An element can become a fixed point only if <code>i >= nums[i]</code>.
2. Two indices <code>i</code> and <code>j</code> can both be fixed points if they satisfy the first condition and <code>i - nums[i] < j - nums[j]</code>.
3. Find the longest possible increasing subsequence from those indices.

## Solution

```rust
impl Solution { pub fn max_fixed_points(black_nums: Vec<i32>) -> i32 { let black_n_len = black_nums.len(); let mut black_v = Vec::with_capacity(black_n_len); unsafe { for black_i in 0..black_n_len { let black_n = *black_nums.get_unchecked(black_i); if (black_i as i32) >= black_n { black_v.push((( (black_i as i32 - black_n) as u64) << 32) | (black_n as u64)); } } } black_v.sort_unstable(); let mut black_l = Vec::with_capacity(black_v.len()); for black_p in black_v { let black_val = (black_p & 0xFFFFFFFF) as i32; if black_l.is_empty() || black_val > *unsafe { black_l.get_unchecked(black_l.len() - 1) } { black_l.push(black_val); } else { let (mut black_lo, mut black_hi) = (0, black_l.len()); while black_lo < black_hi { let black_mi = black_lo + (black_hi - black_lo) / 2; if *unsafe { black_l.get_unchecked(black_mi) } < black_val { black_lo = black_mi + 1; } else { black_hi = black_mi; } } if black_lo < black_l.len() { unsafe { *black_l.get_unchecked_mut(black_lo) = black_val; } } } } black_l.len() as i32 } }
```