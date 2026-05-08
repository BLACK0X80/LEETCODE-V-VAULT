# Longest Mountain in Array

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Dynamic Programming, Enumeration

---

## Problem

<p>You may recall that an array <code>arr</code> is a <strong>mountain array</strong> if and only if:</p>

<ul>
	<li><code>arr.length &gt;= 3</code></li>
	<li>There exists some index <code>i</code> (<strong>0-indexed</strong>) with <code>0 &lt; i &lt; arr.length - 1</code> such that:
	<ul>
		<li><code>arr[0] &lt; arr[1] &lt; ... &lt; arr[i - 1] &lt; arr[i]</code></li>
		<li><code>arr[i] &gt; arr[i + 1] &gt; ... &gt; arr[arr.length - 1]</code></li>
	</ul>
	</li>
</ul>

<p>Given an integer array <code>arr</code>, return <em>the length of the longest subarray, which is a mountain</em>. Return <code>0</code> if there is no mountain subarray.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,1,4,7,3,2,5]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The largest mountain is [1,4,7,3,2] which has length 5.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,2,2]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no mountain.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= arr[i] &lt;= 10<sup>4</sup></code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong></p>

<ul>
	<li>Can you solve it using only one pass?</li>
	<li>Can you solve it in <code>O(1)</code> space?</li>
</ul>



## Solution

```rust
impl Solution { pub fn longest_mountain(arr: Vec<i32>) -> i32 { let (mut res, n) = (0, arr.len()); if n < 3 { return 0; } for i in 1..n-1 { if arr[i-1] < arr[i] && arr[i] > arr[i+1] { let (mut l, mut r) = (i - 1, i + 1); while l > 0 && arr[l-1] < arr[l] { l -= 1; } while r < n - 1 && arr[r+1] < arr[r] { r += 1; } res = res.max(r - l + 1); } } res as i32 } }
```