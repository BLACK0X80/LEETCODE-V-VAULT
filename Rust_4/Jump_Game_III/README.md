# Jump Game III

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search

---

## Problem

<p>Given an array of non-negative integers <code>arr</code>, you are initially positioned at <code>start</code>&nbsp;index of the array. When you are at index <code>i</code>, you can jump&nbsp;to <code>i + arr[i]</code> or <code>i - arr[i]</code>, check if you can reach&nbsp;<strong>any</strong> index with value 0.</p>

<p>Notice that you can not jump outside of the array at any time.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [4,2,3,0,3,1,2], start = 5
<strong>Output:</strong> true
<strong>Explanation:</strong> 
All possible ways to reach at index 3 with value 0 are: 
index 5 -&gt; index 4 -&gt; index 1 -&gt; index 3 
index 5 -&gt; index 6 -&gt; index 4 -&gt; index 1 -&gt; index 3 
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [4,2,3,0,3,1,2], start = 0
<strong>Output:</strong> true 
<strong>Explanation: 
</strong>One possible way to reach at index 3 with value 0 is: 
index 0 -&gt; index 4 -&gt; index 1 -&gt; index 3
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [3,0,2,1,2], start = 2
<strong>Output:</strong> false
<strong>Explanation: </strong>There is no way to reach at index 1 with value 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= arr[i] &lt;&nbsp;arr.length</code></li>
	<li><code>0 &lt;= start &lt; arr.length</code></li>
</ul>


## Hints

1. Think of BFS to solve the problem.
2. When you reach a position with a value = 0 then return true.

## Solution

```rust
impl Solution { pub fn can_reach(arr: Vec<i32>, start: i32) -> bool { let (mut black_q, mut black_vis) = (std::collections::VecDeque::from([start as usize]), vec![false; arr.len()]); while let Some(black_i) = black_q.pop_front() { if arr[black_i] == 0 { return true; } if !black_vis[black_i] { black_vis[black_i] = true; for &black_step in &[black_i as i32 + arr[black_i], black_i as i32 - arr[black_i]] { if black_step >= 0 && (black_step as usize) < arr.len() { black_q.push_back(black_step as usize); } } } } false } }
```