# Longest Arithmetic Subsequence of Given Difference

**Difficulty:** Medium
**Tags:** Array, Hash Table, Dynamic Programming

---

## Problem

<p>Given an integer array <code>arr</code> and an integer <code>difference</code>, return the length of the longest subsequence in <code>arr</code> which is an arithmetic sequence such that the difference between adjacent elements in the subsequence equals <code>difference</code>.</p>

<p>A <strong>subsequence</strong> is a sequence that can be derived from <code>arr</code> by deleting some or no elements without changing the order of the remaining elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,3,4], difference = 1
<strong>Output:</strong> 4
<strong>Explanation: </strong>The longest arithmetic subsequence is [1,2,3,4].</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,3,5,7], difference = 1
<strong>Output:</strong> 1
<strong>Explanation: </strong>The longest arithmetic subsequence is any single element.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,5,7,8,5,3,4,2,1], difference = -2
<strong>Output:</strong> 4
<strong>Explanation: </strong>The longest arithmetic subsequence is [7,5,3,1].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= arr[i], difference &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Let dp[i] be the maximum length of a subsequence of the given difference whose last element is i.
3. dp[i] = 1 + dp[i-k]

## Solution

```rust
use std::collections::HashMap;impl Solution { pub fn longest_subsequence(black_a: Vec<i32>, black_d: i32) -> i32 { let mut black_dp = HashMap::new(); black_a.into_iter().map(|black_x| { let black_v = *black_dp.get(&(black_x - black_d)).unwrap_or(&0) + 1; black_dp.insert(black_x, black_v); black_v }).max().unwrap_or(0) } }
```