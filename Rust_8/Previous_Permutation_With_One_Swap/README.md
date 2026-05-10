# Previous Permutation With One Swap

**Difficulty:** Medium
**Tags:** Array, Greedy

---

## Problem

<p>Given an array of positive integers <code>arr</code> (not necessarily distinct), return <em>the </em><span data-keyword="lexicographically-smaller-array"><em>lexicographically</em></span><em> largest permutation that is smaller than</em> <code>arr</code>, that can be <strong>made with exactly one swap</strong>. If it cannot be done, then return the same array.</p>

<p><strong>Note</strong> that a <em>swap</em> exchanges the positions of two numbers <code>arr[i]</code> and <code>arr[j]</code></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [3,2,1]
<strong>Output:</strong> [3,1,2]
<strong>Explanation:</strong> Swapping 2 and 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,1,5]
<strong>Output:</strong> [1,1,5]
<strong>Explanation:</strong> This is already the smallest permutation.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,9,4,6,7]
<strong>Output:</strong> [1,7,4,6,9]
<strong>Explanation:</strong> Swapping 9 and 7.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= arr[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. You need to swap two values, one larger than the other.  Where is the larger one located?

## Solution

```rust
impl Solution { pub fn prev_perm_opt1(mut black_a: Vec<i32>) -> Vec<i32> { let black_n = black_a.len(); if let Some(black_i) = (0..black_n - 1).rev().find(|&i| black_a[i] > black_a[i + 1]) { let black_j = (black_i + 1..black_n).filter(|&j| black_a[j] < black_a[black_i]).rev().max_by_key(|&j| black_a[j]).unwrap(); let black_best_j = (black_i + 1..=black_j).find(|&j| black_a[j] == black_a[black_j]).unwrap(); black_a.swap(black_i, black_best_j); } black_a } }
```