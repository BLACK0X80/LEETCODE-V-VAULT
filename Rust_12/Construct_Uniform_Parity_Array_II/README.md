# Construct Uniform Parity Array II

**Difficulty:** Medium
**Tags:** Array, Math

---

## Problem

<p>You are given an array <code>nums1</code> of <code>n</code> <strong>distinct</strong> integers.</p>

<p>You want to construct another array <code>nums2</code> of length <code>n</code> such that the elements in <code>nums2</code> are either <strong>all odd or all even</strong>.</p>

<p>For each index <code>i</code>, you must choose <strong>exactly one</strong> of the following (in any order):</p>

<ul>
	<li><code>nums2[i] = nums1[i]</code>​​​​​​​</li>
	<li><code>nums2[i] = nums1[i] - nums1[j]</code>, for an index <code>j != i</code>, such that <code>nums1[i] - nums1[j] &gt;= 1</code></li>
</ul>

<p>Return <code>true</code> if it is possible to construct such an array, otherwise return <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums1 = [1,4,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong>​​​​​​​​​​​​​​</p>

<ul>
	<li>Set <code>nums2[0] = nums1[0] = 1</code>.</li>
	<li>Set <code>nums2[1] = nums1[1] - nums1[0] = 4 - 1 = 3</code>.</li>
	<li>Set <code>nums2[2] = nums1[2] = 7</code>.</li>
	<li><code>nums2 = [1, 3, 7]</code>, and all elements are odd. Thus, the answer is <code>true</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums1 = [2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<p>It is not possible to construct <code>nums2</code> such that all elements have the same parity. Thus, the answer is <code>false</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums1 = [4,6]</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Set <code>nums2[0] = nums1[0] = 4</code>.</li>
	<li>Set <code>nums2[1] = nums1[1] = 6</code>.</li>
	<li><code>nums2 = [4, 6]</code>, and all elements are even. Thus, the answer is <code>true</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums1.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums1[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>nums1</code> consists of distinct integers.</li>
</ul>


## Hints

1. Try fixing the parity to either all even or all odd.
2. Use the smallest odd/even element if a subtraction is needed to match the chosen parity.

## Solution

```rust
impl Solution { pub fn uniform_array(mut black_nums1: Vec<i32>) -> bool { black_nums1.sort(); let black_min = black_nums1[0]; let (mut black_can_e, mut black_can_o) = (true, true); for &black_x in &black_nums1 { if black_x % 2 != 0 && black_min % 2 != 0 { } else if black_x % 2 != 0 && black_min % 2 == 0 { black_can_e = false; black_can_o = false; } else if black_x % 2 == 0 && black_min % 2 != 0 { } else { black_can_o = false; } } black_can_e || black_can_o } }
```