# Maximize Distance to Closest Person

**Difficulty:** Medium
**Tags:** Array

---

## Problem

<p>You are given an array representing a row of <code>seats</code> where <code>seats[i] = 1</code> represents a person sitting in the <code>i<sup>th</sup></code> seat, and <code>seats[i] = 0</code> represents that the <code>i<sup>th</sup></code> seat is empty <strong>(0-indexed)</strong>.</p>

<p>There is at least one empty seat, and at least one person sitting.</p>

<p>Alex wants to sit in the seat such that the distance between him and the closest person to him is maximized.&nbsp;</p>

<p>Return <em>that maximum distance to the closest person</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/10/distance.jpg" style="width: 650px; height: 257px;" />
<pre>
<strong>Input:</strong> seats = [1,0,0,0,1,0,1]
<strong>Output:</strong> 2
<strong>Explanation: </strong>
If Alex sits in the second open seat (i.e. seats[2]), then the closest person has distance 2.
If Alex sits in any other open seat, the closest person has distance 1.
Thus, the maximum distance to the closest person is 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> seats = [1,0,0,0]
<strong>Output:</strong> 3
<strong>Explanation: </strong>
If Alex sits in the last seat (i.e. seats[3]), the closest person is 3 seats away.
This is the maximum distance possible, so the answer is 3.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> seats = [0,1]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= seats.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>seats[i]</code>&nbsp;is <code>0</code> or&nbsp;<code>1</code>.</li>
	<li>At least one seat is <strong>empty</strong>.</li>
	<li>At least one seat is <strong>occupied</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn max_dist_to_closest(black_s: Vec<i32>) -> i32 { let black_idx: Vec<usize> = black_s.iter().enumerate().filter(|&(_, &v)| v == 1).map(|(i, _)| i).collect(); let mut black_res = black_idx[0].max(black_s.len() - 1 - black_idx.last().unwrap()); for black_i in 1..black_idx.len() { black_res = black_res.max((black_idx[black_i] - black_idx[black_i - 1]) / 2); } black_res as i32 } }
```