# Advantage Shuffle

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Greedy, Sorting

---

## Problem

<p>You are given two integer arrays <code>nums1</code> and <code>nums2</code> both of the same length. The <strong>advantage</strong> of <code>nums1</code> with respect to <code>nums2</code> is the number of indices <code>i</code> for which <code>nums1[i] &gt; nums2[i]</code>.</p>

<p>Return <em>any permutation of </em><code>nums1</code><em> that maximizes its <strong>advantage</strong> with respect to </em><code>nums2</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums1 = [2,7,11,15], nums2 = [1,10,4,11]
<strong>Output:</strong> [2,11,7,15]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums1 = [12,24,8,32], nums2 = [13,25,32,11]
<strong>Output:</strong> [24,32,8,12]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums2.length == nums1.length</code></li>
	<li><code>0 &lt;= nums1[i], nums2[i] &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn advantage_count(mut black_n1: Vec<i32>, black_n2: Vec<i32>) -> Vec<i32> { black_n1.sort(); let mut black_idx: Vec<usize> = (0..black_n2.len()).collect(); black_idx.sort_by_key(|&i| black_n2[i]); let mut black_res = vec![0; black_n2.len()]; let (mut black_l, mut black_r) = (0, black_n1.len() - 1); for &black_i in black_idx.iter().rev() { if black_n1[black_r] > black_n2[black_i] { black_res[black_i] = black_n1[black_r]; black_r = black_r.saturating_sub(1); } else { black_res[black_i] = black_n1[black_l]; black_l += 1; } } black_res } }
```