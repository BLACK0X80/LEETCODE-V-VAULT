# Shortest Distance After Road Addition Queries I

**Difficulty:** Medium
**Tags:** Array, Breadth-First Search, Graph Theory

---

## Problem

<p>You are given an integer <code>n</code> and a 2D integer array <code>queries</code>.</p>

<p>There are <code>n</code> cities numbered from <code>0</code> to <code>n - 1</code>. Initially, there is a <strong>unidirectional</strong> road from city <code>i</code> to city <code>i + 1</code> for all <code>0 &lt;= i &lt; n - 1</code>.</p>

<p><code>queries[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> represents the addition of a new <strong>unidirectional</strong> road from city <code>u<sub>i</sub></code> to city <code>v<sub>i</sub></code>. After each query, you need to find the <strong>length</strong> of the <strong>shortest path</strong> from city <code>0</code> to city <code>n - 1</code>.</p>

<p>Return an array <code>answer</code> where for each <code>i</code> in the range <code>[0, queries.length - 1]</code>, <code>answer[i]</code> is the <em>length of the shortest path</em> from city <code>0</code> to city <code>n - 1</code> after processing the <strong>first </strong><code>i + 1</code> queries.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, queries = [[2,4],[0,2],[0,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,2,1]</span></p>

<p><strong>Explanation: </strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/06/28/image8.jpg" style="width: 350px; height: 60px;" /></p>

<p>After the addition of the road from 2 to 4, the length of the shortest path from 0 to 4 is 3.</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/06/28/image9.jpg" style="width: 350px; height: 60px;" /></p>

<p>After the addition of the road from 0 to 2, the length of the shortest path from 0 to 4 is 2.</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/06/28/image10.jpg" style="width: 350px; height: 96px;" /></p>

<p>After the addition of the road from 0 to 4, the length of the shortest path from 0 to 4 is 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, queries = [[0,3],[0,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,1]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/06/28/image11.jpg" style="width: 300px; height: 70px;" /></p>

<p>After the addition of the road from 0 to 3, the length of the shortest path from 0 to 3 is 1.</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/06/28/image12.jpg" style="width: 300px; height: 70px;" /></p>

<p>After the addition of the road from 0 to 2, the length of the shortest path remains 1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= n &lt;= 500</code></li>
	<li><code>1 &lt;= queries.length &lt;= 500</code></li>
	<li><code>queries[i].length == 2</code></li>
	<li><code>0 &lt;= queries[i][0] &lt; queries[i][1] &lt; n</code></li>
	<li><code>1 &lt; queries[i][1] - queries[i][0]</code></li>
	<li>There are no repeated roads among the queries.</li>
</ul>


## Hints

1. Maintain the graph and use an efficient shortest path algorithm after each update.
2. We use BFS/Dijkstra for each query.

## Solution

```rust
impl Solution { pub fn shortest_distance_after_queries(black_n: i32, black_q_list: Vec<Vec<i32>>) -> Vec<i32> { let mut black_adj = (0..black_n-1).map(|black_i| vec![black_i+1]).collect::<Vec<_>>(); black_adj.push(vec![]); black_q_list.into_iter().map(|black_query| { black_adj[black_query[0] as usize].push(black_query[1]); let (mut black_q, mut black_v) = (std::collections::VecDeque::from([(0, 0)]), vec![false; black_n as usize]); black_v[0] = true; while let Some((black_u, black_d)) = black_q.pop_front() { if black_u == (black_n - 1) as usize { return black_d; } for &black_nxt in &black_adj[black_u] { if !black_v[black_nxt as usize] { black_v[black_nxt as usize] = true; black_q.push_back((black_nxt as usize, black_d + 1)); } } } 0 }).collect() } }
```