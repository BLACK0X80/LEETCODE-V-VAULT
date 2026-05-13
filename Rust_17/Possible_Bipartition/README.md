# Possible Bipartition

**Difficulty:** Medium
**Tags:** Depth-First Search, Breadth-First Search, Union-Find, Graph Theory

---

## Problem

<p>We want to split a group of <code>n</code> people (labeled from <code>1</code> to <code>n</code>) into two groups of <strong>any size</strong>. Each person may dislike some other people, and they should not go into the same group.</p>

<p>Given the integer <code>n</code> and the array <code>dislikes</code> where <code>dislikes[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that the person labeled <code>a<sub>i</sub></code> does not like the person labeled <code>b<sub>i</sub></code>, return <code>true</code> <em>if it is possible to split everyone into two groups in this way</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 4, dislikes = [[1,2],[1,3],[2,4]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The first group has [1,4], and the second group has [2,3].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3, dislikes = [[1,2],[1,3],[2,3]]
<strong>Output:</strong> false
<strong>Explanation:</strong> We need at least 3 groups to divide them. We cannot put them in two groups.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 2000</code></li>
	<li><code>0 &lt;= dislikes.length &lt;= 10<sup>4</sup></code></li>
	<li><code>dislikes[i].length == 2</code></li>
	<li><code>1 &lt;= a<sub>i</sub> &lt; b<sub>i</sub> &lt;= n</code></li>
	<li>All the pairs of <code>dislikes</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn possible_bipartition(black_n: i32, black_d: Vec<Vec<i32>>) -> bool { let mut black_adj = vec![vec![]; black_n as usize + 1]; for p in black_d { black_adj[p[0] as usize].push(p[1] as usize); black_adj[p[1] as usize].push(p[0] as usize); } let mut black_c = vec![0; black_n as usize + 1]; for i in 1..=black_n as usize { if black_c[i] == 0 { let mut black_q = std::collections::VecDeque::from([i]); black_c[i] = 1; while let Some(u) = black_q.pop_front() { for &v in &black_adj[u] { if black_c[v] == 0 { black_c[v] = -black_c[u]; black_q.push_back(v); } else if black_c[v] == black_c[u] { return false; } } } } } true } }
```