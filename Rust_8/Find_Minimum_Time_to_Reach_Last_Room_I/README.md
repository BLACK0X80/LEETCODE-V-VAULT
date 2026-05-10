# Find Minimum Time to Reach Last Room I

**Difficulty:** Medium
**Tags:** Array, Graph Theory, Heap (Priority Queue), Matrix, Shortest Path

---

## Problem

<p>There is a dungeon with <code>n x m</code> rooms arranged as a grid.</p>

<p>You are given a 2D array <code>moveTime</code> of size <code>n x m</code>, where <code>moveTime[i][j]</code> represents the <strong>minimum</strong> time in seconds <strong>after</strong> which the room opens and can be moved to. You start from the room <code>(0, 0)</code> at time <code>t = 0</code> and can move to an <strong>adjacent</strong> room. Moving between adjacent rooms takes <em>exactly</em> one second.</p>

<p>Return the <strong>minimum</strong> time to reach the room <code>(n - 1, m - 1)</code>.</p>

<p>Two rooms are <strong>adjacent</strong> if they share a common wall, either <em>horizontally</em> or <em>vertically</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">moveTime = [[0,4],[4,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>The minimum time required is 6 seconds.</p>

<ul>
	<li>At time <code>t == 4</code>, move from room <code>(0, 0)</code> to room <code>(1, 0)</code> in one second.</li>
	<li>At time <code>t == 5</code>, move from room <code>(1, 0)</code> to room <code>(1, 1)</code> in one second.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">moveTime = [[0,0,0],[0,0,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The minimum time required is 3 seconds.</p>

<ul>
	<li>At time <code>t == 0</code>, move from room <code>(0, 0)</code> to room <code>(1, 0)</code> in one second.</li>
	<li>At time <code>t == 1</code>, move from room <code>(1, 0)</code> to room <code>(1, 1)</code> in one second.</li>
	<li>At time <code>t == 2</code>, move from room <code>(1, 1)</code> to room <code>(1, 2)</code> in one second.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">moveTime = [[0,1],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == moveTime.length &lt;= 50</code></li>
	<li><code>2 &lt;= m == moveTime[i].length &lt;= 50</code></li>
	<li><code>0 &lt;= moveTime[i][j] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use shortest path algorithms.

## Solution

```rust
use std::collections::BinaryHeap; impl Solution { pub fn min_time_to_reach(black_mt: Vec<Vec<i32>>) -> i32 { let (black_m, black_n) = (black_mt.len(), black_mt[0].len()); let mut black_d = vec![vec![i32::MAX; black_n]; black_m]; let mut black_h = BinaryHeap::new(); black_d[0][0] = 0; black_h.push((0, 0, 0)); while let Some((black_t, r, c)) = black_h.pop() { let black_t = -black_t; if black_t > black_d[r][c] { continue; } for (dr, dc) in [(0,1),(0,-1),(1,0),(-1,0)] { let (nr, nc) = (r as i32 + dr, c as i32 + dc); if nr >= 0 && nr < black_m as i32 && nc >= 0 && nc < black_n as i32 { let (nr, nc) = (nr as usize, nc as usize); let black_nt = black_t.max(black_mt[nr][nc]) + 1; if black_nt < black_d[nr][nc] { black_d[nr][nc] = black_nt; black_h.push((-black_nt, nr, nc)); } } } } black_d[black_m-1][black_n-1] } }
```