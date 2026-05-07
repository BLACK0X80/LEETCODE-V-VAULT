# Shortest Subarray With OR at Least K II

**Difficulty:** Medium
**Tags:** Array, Bit Manipulation, Sliding Window

---

## Problem

<p>You are given an array <code>nums</code> of <strong>non-negative</strong> integers and an integer <code>k</code>.</p>

<p>An array is called <strong>special</strong> if the bitwise <code>OR</code> of all of its elements is <strong>at least</strong> <code>k</code>.</p>

<p>Return <em>the length of the <strong>shortest</strong> <strong>special</strong> <strong>non-empty</strong> <span data-keyword="subarray-nonempty">subarray</span> of</em> <code>nums</code>, <em>or return</em> <code>-1</code> <em>if no special subarray exists</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The subarray <code>[3]</code> has <code>OR</code> value of <code>3</code>. Hence, we return <code>1</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,8], k = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The subarray <code>[2,1,8]</code> has <code>OR</code> value of <code>11</code>. Hence, we return <code>3</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2], k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The subarray <code>[1]</code> has <code>OR</code> value of <code>1</code>. Hence, we return <code>1</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. For each <code>nums[i]</code>, we can maintain each subarray’s bitwise <code>OR</code> result ending with it.
2. The property of bitwise <code>OR</code> is that it never unsets any bits and only sets new bits
3. So the number of different results for each <code>nums[i]</code> is at most the number of bits 32.

## Solution

```rust
impl Solution { pub fn minimum_subarray_length(black_n: Vec<i32>, black_k: i32) -> i32 { let (mut black_bits, mut black_i, mut black_res) = (vec![0; 32], 0, i32::MAX); for black_j in 0..black_n.len() { for b in 0..32 { if (black_n[black_j] >> b) & 1 == 1 { black_bits[b] += 1; } } while black_i <= black_j && (0..32).fold(0, |acc, b| acc | (if black_bits[b] > 0 { 1 << b } else { 0 })) >= black_k { black_res = black_res.min((black_j - black_i + 1) as i32); for b in 0..32 { if (black_n[black_i] >> b) & 1 == 1 { black_bits[b] -= 1; } } black_i += 1; } } if black_res == i32::MAX { -1 } else { black_res } } }
```