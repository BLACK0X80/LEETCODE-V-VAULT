# Minimum Operations to Make Binary Array Elements Equal to One I

**Difficulty:** Medium
**Tags:** Array, Bit Manipulation, Queue, Sliding Window, Prefix Sum

---

## Problem

<p>You are given a <span data-keyword="binary-array">binary array</span> <code>nums</code>.</p>

<p>You can do the following operation on the array <strong>any</strong> number of times (possibly zero):</p>

<ul>
	<li>Choose <strong>any</strong> 3 <strong>consecutive</strong> elements from the array and <strong>flip</strong> <strong>all</strong> of them.</li>
</ul>

<p><strong>Flipping</strong> an element means changing its value from 0 to 1, and from 1 to 0.</p>

<p>Return the <strong>minimum</strong> number of operations required to make all elements in <code>nums</code> equal to 1. If it is impossible, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,1,1,1,0,0]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong><br />
We can do the following operations:</p>

<ul>
	<li>Choose the elements at indices 0, 1 and 2. The resulting array is <code>nums = [<u><strong>1</strong></u>,<u><strong>0</strong></u>,<u><strong>0</strong></u>,1,0,0]</code>.</li>
	<li>Choose the elements at indices 1, 2 and 3. The resulting array is <code>nums = [1,<u><strong>1</strong></u>,<u><strong>1</strong></u>,<strong><u>0</u></strong>,0,0]</code>.</li>
	<li>Choose the elements at indices 3, 4 and 5. The resulting array is <code>nums = [1,1,1,<strong><u>1</u></strong>,<u><strong>1</strong></u>,<u><strong>1</strong></u>]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,1,1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong><br />
It is impossible to make all elements equal to 1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 1</code></li>
</ul>


## Hints

1. If <code>nums[0]</code> is 0, then the only way to change it to 1 is by doing an operation on the first 3 elements of the array.
2. After Changing <code>nums[0]</code> to 1, use the same logic on the remaining array.

## Solution

```rust
impl Solution { pub fn min_operations(mut black_n: Vec<i32>) -> i32 { let (mut black_res, black_len) = (0, black_n.len()); for i in 0..black_len - 2 { if black_n[i] == 0 { black_res += 1; for j in 0..3 { black_n[i+j] ^= 1; } } } if black_n.iter().all(|&x| x == 1) { black_res } else { -1 } } }
```