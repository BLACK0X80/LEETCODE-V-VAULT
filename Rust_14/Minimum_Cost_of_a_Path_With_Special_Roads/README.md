# Minimum Cost of a Path With Special Roads

**Difficulty:** Medium
**Tags:** Array, Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>You are given an array <code>start</code> where <code>start = [startX, startY]</code> represents your initial position <code>(startX, startY)</code> in a 2D space. You are also given the array <code>target</code> where <code>target = [targetX, targetY]</code> represents your target position <code>(targetX, targetY)</code>.</p>

<p>The <strong>cost</strong> of going from a position <code>(x1, y1)</code> to any other position in the space <code>(x2, y2)</code> is <code>|x2 - x1| + |y2 - y1|</code>.</p>

<p>There are also some <strong>special roads</strong>. You are given a 2D array <code>specialRoads</code> where <code>specialRoads[i] = [x1<sub>i</sub>, y1<sub>i</sub>, x2<sub>i</sub>, y2<sub>i</sub>, cost<sub>i</sub>]</code> indicates that the <code>i<sup>th</sup></code> special road goes in <strong>one direction</strong> from <code>(x1<sub>i</sub>, y1<sub>i</sub>)</code> to <code>(x2<sub>i</sub>, y2<sub>i</sub>)</code> with a cost equal to <code>cost<sub>i</sub></code>. You can use each special road any number of times.</p>

<p>Return the <strong>minimum</strong> cost required to go from <code>(startX, startY)</code> to <code>(targetX, targetY)</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">start = [1,1], target = [4,5], specialRoads = [[1,2,3,3,2],[3,4,4,5,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<ol>
	<li>(1,1) to (1,2) with a cost of |1 - 1| + |2 - 1| = 1.</li>
	<li>(1,2) to (3,3). Use <code><span class="example-io">specialRoads[0]</span></code><span class="example-io"> with</span><span class="example-io"> the cost 2.</span></li>
	<li><span class="example-io">(3,3) to (3,4) with a cost of |3 - 3| + |4 - 3| = 1.</span></li>
	<li><span class="example-io">(3,4) to (4,5). Use </span><code><span class="example-io">specialRoads[1]</span></code><span class="example-io"> with the cost</span><span class="example-io"> 1.</span></li>
</ol>

<p><span class="example-io">So the total cost is 1 + 2 + 1 + 1 = 5.</span></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">start = [3,2], target = [5,7], specialRoads = [[5,7,3,2,1],[3,2,3,4,4],[3,3,5,5,5],[3,4,5,6,6]]</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<p>It is optimal not to use any special edges and go directly from the starting to the ending position with a cost |5 - 3| + |7 - 2| = 7.</p>

<p>Note that the <span class="example-io"><code>specialRoads[0]</code> is directed from (5,7) to (3,2).</span></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">start = [1,1], target = [10,4], specialRoads = [[4,2,1,1,3],[1,2,7,4,4],[10,3,6,1,2],[6,1,1,2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<ol>
	<li>(1,1) to (1,2) with a cost of |1 - 1| + |2 - 1| = 1.</li>
	<li>(1,2) to (7,4). Use <code><span class="example-io">specialRoads[1]</span></code><span class="example-io"> with the cost</span><span class="example-io"> 4.</span></li>
	<li>(7,4) to (10,4) with a cost of |10 - 7| + |4 - 4| = 3.</li>
</ol>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>start.length == target.length == 2</code></li>
	<li><code>1 &lt;= startX &lt;= targetX &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= startY &lt;= targetY &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= specialRoads.length &lt;= 200</code></li>
	<li><code>specialRoads[i].length == 5</code></li>
	<li><code>startX &lt;= x1<sub>i</sub>, x2<sub>i</sub> &lt;= targetX</code></li>
	<li><code>startY &lt;= y1<sub>i</sub>, y2<sub>i</sub> &lt;= targetY</code></li>
	<li><code>1 &lt;= cost<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. It can be proven that it is optimal to go only to the positions that are either the start or the end of a special road or the target position.
2. Consider all positions given to you as nodes in a graph, and the edges of the graph are the special roads.
3. Now the problem is equivalent to finding the shortest path in a directed graph.

## Solution

```rust
impl Solution { pub fn minimum_cost(black_s: Vec<i32>, black_t: Vec<i32>, black_sr: Vec<Vec<i32>>) -> i32 { let mut black_dist = std::collections::HashMap::from([((black_s[0], black_s[1]), 0)]); let mut black_pq = std::collections::BinaryHeap::from([(0, black_s[0], black_s[1])]); let mut black_ans = (black_t[0] - black_s[0]).abs() + (black_t[1] - black_s[1]).abs(); while let Some((black_d, black_x, black_y)) = black_pq.pop() { let black_d = -black_d; if black_d > *black_dist.get(&(black_x, black_y)).unwrap_or(&i32::MAX) { continue; } black_ans = black_ans.min(black_d + (black_t[0] - black_x).abs() + (black_t[1] - black_y).abs()); for black_r in &black_sr { let black_nd = black_d + (black_r[0] - black_x).abs() + (black_r[1] - black_y).abs() + black_r[4]; if black_nd < *black_dist.get(&(black_r[2], black_r[3])).unwrap_or(&i32::MAX) { black_dist.insert((black_r[2], black_r[3]), black_nd); black_pq.push((-black_nd, black_r[2], black_r[3])); } } } black_ans } }
```