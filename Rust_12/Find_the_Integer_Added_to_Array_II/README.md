# Find the Integer Added to Array II

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Sorting, Enumeration

---

## Problem

<p>You are given two integer arrays <code>nums1</code> and <code>nums2</code>.</p>

<p>From <code>nums1</code> two elements have been removed, and all other elements have been increased (or decreased in the case of negative) by an integer, represented by the variable <code>x</code>.</p>

<p>As a result, <code>nums1</code> becomes <strong>equal</strong> to <code>nums2</code>. Two arrays are considered <strong>equal</strong> when they contain the same integers with the same frequencies.</p>

<p>Return the <strong>minimum</strong> possible integer<em> </em><code>x</code><em> </em>that achieves this equivalence.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io" style="
    font-family: Menlo,sans-serif;
    font-size: 0.85rem;
">nums1 = [4,20,16,12,8], nums2 = [14,18,10]</span></p>

<p><strong>Output:</strong> <span class="example-io" style="
    font-family: Menlo,sans-serif;
    font-size: 0.85rem;
">-2</span></p>

<p><strong>Explanation:</strong></p>

<p>After removing elements at indices <code>[0,4]</code> and adding -2, <code>nums1</code> becomes <code>[18,14,10]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io" style="
    font-family: Menlo,sans-serif;
    font-size: 0.85rem;
">nums1 = [3,5,5,3], nums2 = [7,7]</span></p>

<p><strong>Output:</strong> <span class="example-io" style="
    font-family: Menlo,sans-serif;
    font-size: 0.85rem;
">2</span></p>

<p><strong>Explanation:</strong></p>

<p>After removing elements at indices <code>[0,3]</code> and adding 2, <code>nums1</code> becomes <code>[7,7]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= nums1.length &lt;= 200</code></li>
	<li><code>nums2.length == nums1.length - 2</code></li>
	<li><code>0 &lt;= nums1[i], nums2[i] &lt;= 1000</code></li>
	<li>The test cases are generated in a way that there is an integer <code>x</code> such that <code>nums1</code> can become equal to <code>nums2</code> by removing two elements and adding <code>x</code> to each element of <code>nums1</code>.</li>
</ul>


## Hints

1. Try all possibilities to remove 2 elements from <code>nums1</code>.
2. <code>x</code> should be equal to <code>min(nums2) - min(nums1)</code>, check it naively.

## Solution

```rust
impl Solution { pub fn minimum_added_integer(mut black_n1: Vec<i32>, mut black_n2: Vec<i32>) -> i32 { black_n1.sort_unstable(); black_n2.sort_unstable(); [black_n1[2], black_n1[1], black_n1[0]].iter().filter_map(|&v| { let black_x = black_n2[0] - v; let (mut black_i, mut black_j, mut black_err) = (0, 0, 0); while black_i < black_n1.len() && black_j < black_n2.len() { if black_n1[black_i] + black_x == black_n2[black_j] { black_j += 1; } else { black_err += 1; } black_i += 1; } if black_j == black_n2.len() { Some(black_x) } else { None } }).min().unwrap() } }
```