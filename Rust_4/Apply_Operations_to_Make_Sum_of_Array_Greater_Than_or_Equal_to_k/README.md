# Apply Operations to Make Sum of Array Greater Than or Equal to k

**Difficulty:** Medium
**Tags:** Math, Greedy, Enumeration

---

## Problem

<p>You are given a <strong>positive</strong> integer <code>k</code>. Initially, you have an array <code>nums = [1]</code>.</p>

<p>You can perform <strong>any</strong> of the following operations on the array <strong>any</strong> number of times (<strong>possibly zero</strong>):</p>

<ul>
	<li>Choose any element in the array and <strong>increase</strong> its value by <code>1</code>.</li>
	<li>Duplicate any element in the array and add it to the end of the array.</li>
</ul>

<p>Return <em>the <strong>minimum</strong> number of operations required to make the <strong>sum</strong> of elements of the final array greater than or equal to </em><code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">k = 11</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>We can do the following operations on the array <code>nums = [1]</code>:</p>

<ul>
	<li>Increase the element by <code>1</code> three times. The resulting array is <code>nums = [4]</code>.</li>
	<li>Duplicate the element two times. The resulting array is <code>nums = [4,4,4]</code>.</li>
</ul>

<p>The sum of the final array is <code>4 + 4 + 4 = 12</code> which is greater than or equal to <code>k = 11</code>.<br />
The total number of operations performed is <code>3 + 2 = 5</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The sum of the original array is already greater than or equal to <code>1</code>, so no operations are needed.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. It is optimal to make all the increase operations first and all the duplicate operations last.
2. Iterate over all possible number of increase operations that can be done and find the corresponding number of duplicate operations.

## Solution

```rust
impl Solution { pub fn min_operations(k: i32) -> i32 { if k == 1 { return 0; } let mut black_min = k - 1; for i in 1..k { let ops = (i - 1) + (k + i - 1) / i - 1; if ops < black_min { black_min = ops; } if i * i > k { break; } } black_min } }
```