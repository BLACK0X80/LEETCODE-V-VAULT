# Frequency of the Most Frequent Element

**Difficulty:** Medium
**Tags:** Array, Binary Search, Greedy, Sliding Window, Sorting, Prefix Sum

---

## Problem

<p>The <strong>frequency</strong> of an element is the number of times it occurs in an array.</p>

<p>You are given an integer array <code>nums</code> and an integer <code>k</code>. In one operation, you can choose an index of <code>nums</code> and increment the element at that index by <code>1</code>.</p>

<p>Return <em>the <strong>maximum possible frequency</strong> of an element after performing <strong>at most</strong> </em><code>k</code><em> operations</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,4], k = 5
<strong>Output:</strong> 3<strong>
Explanation:</strong> Increment the first element three times and the second element two times to make nums = [4,4,4].
4 has a frequency of 3.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,4,8,13], k = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are multiple optimal solutions:
- Increment the first element three times to make nums = [4,4,8,13]. 4 has a frequency of 2.
- Increment the second element four times to make nums = [1,8,8,13]. 8 has a frequency of 2.
- Increment the third element five times to make nums = [1,4,13,13]. 13 has a frequency of 2.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,9,6], k = 2
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Note that you can try all values in a brute force manner and find the maximum frequency of that value.
2. To find the maximum frequency of a value consider the biggest elements smaller than or equal to this value

## Solution

```rust
impl Solution { pub fn max_frequency(mut black_n: Vec<i32>, black_k: i32) -> i32 { black_n.sort_unstable(); let (mut black_l, mut black_s, mut black_res) = (0, 0i64, 0); for black_r in 0..black_n.len() { black_s += black_n[black_r] as i64; while (black_n[black_r] as i64 * (black_r - black_l + 1) as i64) - black_s > black_k as i64 { black_s -= black_n[black_l] as i64; black_l += 1; } black_res = black_res.max(black_r - black_l + 1); } black_res as i32 } }
```