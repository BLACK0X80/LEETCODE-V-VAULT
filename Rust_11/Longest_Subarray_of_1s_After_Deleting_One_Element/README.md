# Longest Subarray of 1's After Deleting One Element

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Sliding Window

---

## Problem

<p>Given a binary array <code>nums</code>, you should delete one element from it.</p>

<p>Return <em>the size of the longest non-empty subarray containing only </em><code>1</code><em>&#39;s in the resulting array</em>. Return <code>0</code> if there is no such subarray.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,0,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1&#39;s.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,1,1,1,0,1,1,0,1]
<strong>Output:</strong> 5
<strong>Explanation:</strong> After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1&#39;s is [1,1,1,1,1].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> You must delete one element.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums[i]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. Maintain a sliding window where there is at most one zero in it.

## Solution

```rust
impl Solution { pub fn longest_subarray(black_n: Vec<i32>) -> i32 { let (mut black_l, mut black_z, mut black_res) = (0, 0, 0); for black_r in 0..black_n.len() { if black_n[black_r] == 0 { black_z += 1; } while black_z > 1 { if black_n[black_l] == 0 { black_z -= 1; } black_l += 1; } black_res = black_res.max(black_r - black_l); } black_res as i32 } }
```