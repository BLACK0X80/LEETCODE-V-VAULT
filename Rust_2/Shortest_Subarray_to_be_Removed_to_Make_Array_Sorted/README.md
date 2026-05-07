# Shortest Subarray to be Removed to Make Array Sorted

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Stack, Monotonic Stack

---

## Problem

<p>Given an integer array <code>arr</code>, remove a subarray (can be empty) from <code>arr</code> such that the remaining elements in <code>arr</code> are <strong>non-decreasing</strong>.</p>

<p>Return <em>the length of the shortest subarray to remove</em>.</p>

<p>A <strong>subarray</strong> is a contiguous subsequence of the array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,3,10,4,2,3,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The shortest subarray we can remove is [10,4,2] of length 3. The remaining elements after that will be [1,2,3,3,5] which are sorted.
Another correct solution is to remove the subarray [3,10,4].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [5,4,3,2,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Since the array is strictly decreasing, we can only keep a single element. Therefore we need to remove a subarray of length 4, either [5,4,3,2] or [4,3,2,1].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array is already non-decreasing. We do not need to remove any elements.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= arr[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. The key is to find the longest non-decreasing subarray starting with the first element or ending with the last element, respectively.
2. After removing some subarray, the result is the concatenation of a sorted prefix and a sorted suffix, where the last element of the prefix is smaller than the first element of the suffix.

## Solution

```rust
impl Solution { pub fn find_length_of_shortest_subarray(black_a: Vec<i32>) -> i32 { let black_n = black_a.len(); let (mut black_l, mut black_r) = (0, black_n - 1); while black_l + 1 < black_n && black_a[black_l] <= black_a[black_l + 1] { black_l += 1; } if black_l == black_n - 1 { return 0; } while black_r > 0 && black_a[black_r - 1] <= black_a[black_r] { black_r -= 1; } let mut black_ans = (black_n - black_l - 1).min(black_r) as i32; let (mut black_i, mut black_j) = (0, black_r); while black_i <= black_l && black_j < black_n { if black_a[black_i] <= black_a[black_j] { black_ans = black_ans.min((black_j - black_i - 1) as i32); black_i += 1; } else { black_j += 1; } } black_ans } }
```