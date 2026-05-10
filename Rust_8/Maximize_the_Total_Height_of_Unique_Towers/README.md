# Maximize the Total Height of Unique Towers

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given an array <code>maximumHeight</code>, where <code>maximumHeight[i]</code> denotes the <strong>maximum</strong> height the <code>i<sup>th</sup></code> tower can be assigned.</p>

<p>Your task is to assign a height to each tower so that:</p>

<ol>
	<li>The height of the <code>i<sup>th</sup></code> tower is a positive integer and does not exceed <code>maximumHeight[i]</code>.</li>
	<li>No two towers have the same height.</li>
</ol>

<p>Return the <strong>maximum</strong> possible total sum of the tower heights. If it&#39;s not possible to assign heights, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> maximumHeight<span class="example-io"> = [2,3,4,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>We can assign heights in the following way: <code>[1, 2, 4, 3]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> maximumHeight<span class="example-io"> = [15,10]</span></p>

<p><strong>Output:</strong> <span class="example-io">25</span></p>

<p><strong>Explanation:</strong></p>

<p>We can assign heights in the following way: <code>[15, 10]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> maximumHeight<span class="example-io"> = [2,2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>It&#39;s impossible to assign positive heights to each index so that no two towers have the same height.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= maximumHeight.length&nbsp;&lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= maximumHeight[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Sort the array <code>maximumHeight</code> in descending order.
2. After sorting, it can be seen that the maximum height that we can assign to the <code>i<sup>th</sup></code> element is <code>min(maximumHeight[i], maximumHeight[i - 1] - 1)</code>.

## Solution

```rust
impl Solution { pub fn maximum_total_sum(mut black_h: Vec<i32>) -> i64 { black_h.sort_unstable_by(|a, b| b.cmp(a)); let (mut black_curr, mut black_sum) = (i32::MAX, 0i64); for h in black_h { black_curr = black_curr.min(h); if black_curr <= 0 { return -1; } black_sum += black_curr as i64; black_curr -= 1; } black_sum } }
```