# Minimum Jumps to Reach Home

**Difficulty:** Medium
**Tags:** Array, Hash Table, Breadth-First Search

---

## Problem

<p>A certain bug&#39;s home is on the x-axis at position <code>x</code>. Help them get there from position <code>0</code>.</p>

<p>The bug jumps according to the following rules:</p>

<ul>
	<li>It can jump exactly <code>a</code> positions <strong>forward</strong> (to the right).</li>
	<li>It can jump exactly <code>b</code> positions <strong>backward</strong> (to the left).</li>
	<li>It cannot jump backward twice in a row.</li>
	<li>It cannot jump to any <code>forbidden</code> positions.</li>
</ul>

<p>The bug may jump forward <strong>beyond</strong> its home, but it <strong>cannot jump</strong> to positions numbered with <strong>negative</strong> integers.</p>

<p>Given an array of integers <code>forbidden</code>, where <code>forbidden[i]</code> means that the bug cannot jump to the position <code>forbidden[i]</code>, and integers <code>a</code>, <code>b</code>, and <code>x</code>, return <em>the minimum number of jumps needed for the bug to reach its home</em>. If there is no possible sequence of jumps that lands the bug on position <code>x</code>, return <code>-1.</code></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9
<strong>Output:</strong> 3
<strong>Explanation:</strong> 3 jumps forward (0 -&gt; 3 -&gt; 6 -&gt; 9) will get the bug home.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11
<strong>Output:</strong> -1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7
<strong>Output:</strong> 2
<strong>Explanation:</strong> One jump forward (0 -&gt; 16) then one jump backward (16 -&gt; 7) will get the bug home.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= forbidden.length &lt;= 1000</code></li>
	<li><code>1 &lt;= a, b, forbidden[i] &lt;= 2000</code></li>
	<li><code>0 &lt;= x &lt;= 2000</code></li>
	<li>All the elements in <code>forbidden</code> are distinct.</li>
	<li>Position <code>x</code> is not forbidden.</li>
</ul>


## Hints

1. Think of the line as a graph
2. to handle the no double back jumps condition you can handle it by holding the state of your previous jump

## Solution

```rust
impl Solution { pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 { let (mut black_q, mut black_vis, black_f, black_limit) = (std::collections::VecDeque::from([(0, 0, false)]), std::collections::HashSet::new(), forbidden.into_iter().collect::<std::collections::HashSet<i32>>(), 6000); black_vis.insert((0, false)); while let Some((black_curr, black_dist, black_back)) = black_q.pop_front() { if black_curr == x { return black_dist; } let mut black_nexts = vec![(black_curr + a, false)]; if !black_back && black_curr - b >= 0 { black_nexts.push((black_curr - b, true)); } for (black_next, black_is_back) in black_nexts { if black_next <= black_limit && !black_f.contains(&black_next) && !black_vis.contains(&(black_next, black_is_back)) { black_vis.insert((black_next, black_is_back)); black_q.push_back((black_next, black_dist + 1, black_is_back)); } } } -1 } }
```