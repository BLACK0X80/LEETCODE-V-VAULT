# Find K Closest Elements

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Sliding Window, Sorting, Heap (Priority Queue)

---

## Problem

<p>Given a <strong>sorted</strong> integer array <code>arr</code>, two integers <code>k</code> and <code>x</code>, return the <code>k</code> closest integers to <code>x</code> in the array. The result should also be sorted in ascending order.</p>

<p>An integer <code>a</code> is closer to <code>x</code> than an integer <code>b</code> if:</p>

<ul>
	<li><code>|a - x| &lt; |b - x|</code>, or</li>
	<li><code>|a - x| == |b - x|</code> and <code>a &lt; b</code></li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">arr = [1,2,3,4,5], k = 4, x = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2,3,4]</span></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">arr = [1,1,2,3,4,5], k = 4, x = -1</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,1,2,3]</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= arr.length</code></li>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>4</sup></code></li>
	<li><code>arr</code> is sorted in <strong>ascending</strong> order.</li>
	<li><code>-10<sup>4</sup> &lt;= arr[i], x &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn find_closest_elements(black_arr: Vec<i32>, black_k: i32, black_x: i32) -> Vec<i32> { let (mut black_l, mut black_h) = (0, black_arr.len() - black_k as usize); while black_l < black_h { let black_m = black_l + (black_h - black_l) / 2; if black_x - black_arr[black_m] > black_arr[black_m + black_k as usize] - black_x { black_l = black_m + 1; } else { black_h = black_m; } } black_arr[black_l..black_l + black_k as usize].to_vec() } }
```