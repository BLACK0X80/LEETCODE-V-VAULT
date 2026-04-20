# Find the Power of K-Size Subarrays II

**Difficulty:** Medium
**Tags:** Array, Sliding Window

---

## Problem

<p>You are given an array of integers <code>nums</code> of length <code>n</code> and a <em>positive</em> integer <code>k</code>.</p>

<p>The <strong>power</strong> of an array is defined as:</p>

<ul>
	<li>Its <strong>maximum</strong> element if <em>all</em> of its elements are <strong>consecutive</strong> and <strong>sorted</strong> in <strong>ascending</strong> order.</li>
	<li>-1 otherwise.</li>
</ul>

<p>You need to find the <strong>power</strong> of all <span data-keyword="subarray-nonempty">subarrays</span> of <code>nums</code> of size <code>k</code>.</p>

<p>Return an integer array <code>results</code> of size <code>n - k + 1</code>, where <code>results[i]</code> is the <em>power</em> of <code>nums[i..(i + k - 1)]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4,3,2,5], k = 3</span></p>

<p><strong>Output:</strong> [3,4,-1,-1,-1]</p>

<p><strong>Explanation:</strong></p>

<p>There are 5 subarrays of <code>nums</code> of size 3:</p>

<ul>
	<li><code>[1, 2, 3]</code> with the maximum element 3.</li>
	<li><code>[2, 3, 4]</code> with the maximum element 4.</li>
	<li><code>[3, 4, 3]</code> whose elements are <strong>not</strong> consecutive.</li>
	<li><code>[4, 3, 2]</code> whose elements are <strong>not</strong> sorted.</li>
	<li><code>[3, 2, 5]</code> whose elements are <strong>not</strong> consecutive.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,2,2,2,2], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">[-1,-1]</span></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,2,3,2,3,2], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[-1,3,-1,3,-1]</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. Let <code>dp[i]</code> denote the length of the longest subarray ending at index <code>i</code> that has consecutive and sorted elements.
2. Use a TreeMap with a sliding window to check if there are <code>k</code> elements in the subarray ending at index <code>i</code>.
3. If TreeMap has less than <code>k</code> elements and <code>dp[i] < k</code>, the subarray has power equal to -1.
4. Is it possible to achieve <code>O(nums.length)</code> using a Stack?

## Solution

```rust
impl Solution { pub fn results_array(black_n: Vec<i32>, black_k: i32) -> Vec<i32> { let (mut black_res, mut black_c, black_k_u) = (Vec::with_capacity(black_n.len() - black_k as usize + 1), 0, black_k as usize); for i in 0..black_n.len() { black_c = if i > 0 && black_n[i] == black_n[i-1] + 1 { black_c + 1 } else { 1 }; if i >= black_k_u - 1 { black_res.push(if black_c >= black_k { black_n[i] } else { -1 }); } } black_res } }
```