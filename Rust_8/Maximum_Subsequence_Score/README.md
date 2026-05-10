# Maximum Subsequence Score

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code> of equal length <code>n</code> and a positive integer <code>k</code>. You must choose a <strong>subsequence</strong> of indices from <code>nums1</code> of length <code>k</code>.</p>

<p>For chosen indices <code>i<sub>0</sub></code>, <code>i<sub>1</sub></code>, ..., <code>i<sub>k - 1</sub></code>, your <strong>score</strong> is defined as:</p>

<ul>
	<li>The sum of the selected elements from <code>nums1</code> multiplied with the <strong>minimum</strong> of the selected elements from <code>nums2</code>.</li>
	<li>It can defined simply as: <code>(nums1[i<sub>0</sub>] + nums1[i<sub>1</sub>] +...+ nums1[i<sub>k - 1</sub>]) * min(nums2[i<sub>0</sub>] , nums2[i<sub>1</sub>], ... ,nums2[i<sub>k - 1</sub>])</code>.</li>
</ul>

<p>Return <em>the <strong>maximum</strong> possible score.</em></p>

<p>A <strong>subsequence</strong> of indices of an array is a set that can be derived from the set <code>{0, 1, ..., n-1}</code> by deleting some or no elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,3,3,2], nums2 = [2,1,3,4], k = 3
<strong>Output:</strong> 12
<strong>Explanation:</strong> 
The four possible subsequence scores are:
- We choose the indices 0, 1, and 2 with score = (1+3+3) * min(2,1,3) = 7.
- We choose the indices 0, 1, and 3 with score = (1+3+2) * min(2,1,4) = 6. 
- We choose the indices 0, 2, and 3 with score = (1+3+2) * min(2,3,4) = 12. 
- We choose the indices 1, 2, and 3 with score = (3+3+2) * min(1,3,4) = 8.
Therefore, we return the max score, which is 12.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [4,2,3,1,1], nums2 = [7,5,10,9,6], k = 1
<strong>Output:</strong> 30
<strong>Explanation:</strong> 
Choosing index 2 is optimal: nums1[2] * nums2[2] = 3 * 10 = 30 is the maximum possible score.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums1.length == nums2.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums1[i], nums2[j] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. How can we use sorting here?
2. Try sorting the two arrays based on second array.
3. Loop through nums2 and compute the max product given the minimum is nums2[i]. Update the answer accordingly.

## Solution

```rust
impl Solution { pub fn max_score(black_n1: Vec<i32>, black_n2: Vec<i32>, black_k: i32) -> i64 { let mut black_combined: Vec<(i32, i32)> = black_n2.into_iter().zip(black_n1.into_iter()).collect(); black_combined.sort_unstable_by_key(|black_pair| -black_pair.0); let mut black_h = std::collections::BinaryHeap::new(); let (mut black_sum, mut black_res) = (0i64, 0i64); for (black_min_val, black_val) in black_combined { black_sum += black_val as i64; black_h.push(std::cmp::Reverse(black_val)); if black_h.len() > black_k as usize { if let Some(std::cmp::Reverse(black_min)) = black_h.pop() { black_sum -= black_min as i64; } } if black_h.len() == black_k as usize { black_res = black_res.max(black_sum * black_min_val as i64); } } black_res } }
```