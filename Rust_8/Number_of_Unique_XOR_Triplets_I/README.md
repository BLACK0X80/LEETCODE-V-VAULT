# Number of Unique XOR Triplets I

**Difficulty:** Medium
**Tags:** Array, Math, Bit Manipulation

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code>, where <code>nums</code> is a <strong><span data-keyword="permutation">permutation</span></strong> of the numbers in the range <code>[1, n]</code>.</p>

<p>A <strong>XOR triplet</strong> is defined as the XOR of three elements <code>nums[i] XOR nums[j] XOR nums[k]</code> where <code>i &lt;= j &lt;= k</code>.</p>

<p>Return the number of <strong>unique</strong> XOR triplet values from all possible triplets <code>(i, j, k)</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible XOR triplet values are:</p>

<ul>
	<li><code>(0, 0, 0) &rarr; 1 XOR 1 XOR 1 = 1</code></li>
	<li><code>(0, 0, 1) &rarr; 1 XOR 1 XOR 2 = 2</code></li>
	<li><code>(0, 1, 1) &rarr; 1 XOR 2 XOR 2 = 1</code></li>
	<li><code>(1, 1, 1) &rarr; 2 XOR 2 XOR 2 = 2</code></li>
</ul>

<p>The unique XOR values are <code>{1, 2}</code>, so the output is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible XOR triplet values include:</p>

<ul>
	<li><code>(0, 0, 0) &rarr; 3 XOR 3 XOR 3 = 3</code></li>
	<li><code>(0, 0, 1) &rarr; 3 XOR 3 XOR 1 = 1</code></li>
	<li><code>(0, 0, 2) &rarr; 3 XOR 3 XOR 2 = 2</code></li>
	<li><code>(0, 1, 2) &rarr; 3 XOR 1 XOR 2 = 0</code></li>
</ul>

<p>The unique XOR values are <code>{0, 1, 2, 3}</code>, so the output is 4.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= n</code></li>
	<li><code>nums</code> is a permutation of integers from <code>1</code> to <code>n</code>.</li>
</ul>


## Hints

1. What is the maximum and minimum value we can obtain using the given numbers?
2. Can we generate all numbers within that range?
3. For <code>n >= 3</code> we can obtain all numbers in <code>[0, 2^(msb(n) + 1) - 1]</code>, where <code>msb(n)</code> is the index of the most significant bit in <code>n</code>’s binary representation (i.e., the highest power of 2 less than or equal to <code>n</code>). Handle the case when <code>n <= 2</code> separately.

## Solution

```rust
impl Solution { pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 { let n = nums.len() as i32; let mut black_max = 0; for &x in &nums { black_max = black_max.max(x); } let mut black_limit = 1; while black_limit <= black_max { black_limit <<= 1; } if n < 3 { let mut black_set = std::collections::HashSet::new(); for i in 0..nums.len() { for j in i..nums.len() { for k in j..nums.len() { black_set.insert(nums[i] ^ nums[j] ^ nums[k]); } } } return black_set.len() as i32; } black_limit } }
```