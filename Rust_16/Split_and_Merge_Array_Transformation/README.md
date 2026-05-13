# Split and Merge Array Transformation

**Difficulty:** Medium
**Tags:** Array, Hash Table, Breadth-First Search

---

## Problem

<p>You are given two integer arrays <code>nums1</code> and <code>nums2</code>, each of length <code>n</code>. You may perform the following <strong>split-and-merge operation</strong> on <code>nums1</code> any number of times:</p>

<ol>
	<li>Choose a subarray <code>nums1[L..R]</code>.</li>
	<li>Remove that subarray, leaving the prefix <code>nums1[0..L-1]</code> (empty if <code>L = 0</code>) and the suffix <code>nums1[R+1..n-1]</code> (empty if <code>R = n - 1</code>).</li>
	<li>Re-insert the removed subarray (in its original order) at <strong>any</strong> position in the remaining array (i.e., between any two elements, at the very start, or at the very end).</li>
</ol>

<p>Return the <strong>minimum</strong> number of <strong>split-and-merge operations</strong> needed to transform <code>nums1</code> into <code>nums2</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums1 = [3,1,2], nums2 = [1,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Split out the subarray <code>[3]</code> (<code>L = 0</code>, <code>R = 0</code>); the remaining array is <code>[1,2]</code>.</li>
	<li>Insert <code>[3]</code> at the end; the array becomes <code>[1,2,3]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums1 = </span>[1,1,2,3,4,5]<span class="example-io">, nums2 = </span>[5,4,3,2,1,1]</p>

<p><strong>Output: </strong>3</p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Remove <code>[1,1,2]</code> at indices <code>0 - 2</code>; remaining is <code>[3,4,5]</code>; insert <code>[1,1,2]</code> at position <code>2</code>, resulting in <code>[3,4,1,1,2,5]</code>.</li>
	<li>Remove <code>[4,1,1]</code> at indices <code>1 - 3</code>; remaining is <code>[3,2,5]</code>; insert <code>[4,1,1]</code> at position <code>3</code>, resulting in <code>[3,2,5,4,1,1]</code>.</li>
	<li>Remove <code>[3,2]</code> at indices <code>0 - 1</code>; remaining is <code>[5,4,1,1]</code>; insert <code>[3,2]</code> at position <code>2</code>, resulting in <code>[5,4,3,2,1,1]</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums1.length == nums2.length &lt;= 6</code></li>
	<li><code>-10<sup>5</sup> &lt;= nums1[i], nums2[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>nums2</code> is a <strong>permutation</strong> of <code>nums1</code>.</li>
</ul>


## Hints

1. Use <code>BFS</code> over the space of array states, starting from <code>nums1</code> and aiming for <code>nums2</code>.
2. Represent each state as an array (or tuple) and enqueue it alongside its current operation count.
3. Maintain a visited set (e.g. a hash set or dictionary keyed by the state) to avoid revisiting the same configuration.
4. For each dequeued state, generate all possible "split-and-merge" successors by choosing every valid subarray <code>[L..R]</code>, removing it, and inserting it at every possible position.
5. Stop as soon as you dequeue <code>nums2</code>, and return its associated operation count.

## Solution

```rust
use std::collections::{VecDeque, HashSet}; impl Solution { pub fn min_split_merge(black_n1: Vec<i32>, black_n2: Vec<i32>) -> i32 { let mut black_q = VecDeque::from([(black_n1, 0)]); let mut black_v = HashSet::from([black_q[0].0.clone()]); while let Some((black_curr, black_d)) = black_q.pop_front() { if black_curr == black_n2 { return black_d; } let black_sz = black_curr.len(); for black_l in 0..black_sz { for black_r in black_l..black_sz { let mut black_sub = black_curr[black_l..=black_r].to_vec(); let mut black_rem = [&black_curr[..black_l], &black_curr[black_r + 1..]].concat(); for black_p in 0..=black_rem.len() { let mut black_next = black_rem.clone(); for (black_i, black_val) in black_sub.iter().enumerate() { black_next.insert(black_p + black_i, *black_val); } if black_v.insert(black_next.clone()) { black_q.push_back((black_next, black_d + 1)); } } } } } -1 } }
```