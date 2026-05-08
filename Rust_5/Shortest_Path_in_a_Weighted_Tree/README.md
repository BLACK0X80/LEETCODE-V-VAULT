# Shortest Path in a Weighted Tree

**Difficulty:** Hard
**Tags:** Array, Tree, Depth-First Search, Binary Indexed Tree, Segment Tree

---

## Problem

<p>You are given an integer <code>n</code> and an undirected, weighted tree rooted at node 1 with <code>n</code> nodes numbered from 1 to <code>n</code>. This is represented by a 2D array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]</code> indicates an undirected edge from node <code>u<sub>i</sub></code> to <code>v<sub>i</sub></code> with weight <code>w<sub>i</sub></code>.</p>

<p>You are also given a 2D integer array <code>queries</code> of length <code>q</code>, where each <code>queries[i]</code> is either:</p>

<ul>
	<li><code>[1, u, v, w&#39;]</code> &ndash; <strong>Update</strong> the weight of the edge between nodes <code>u</code> and <code>v</code> to <code>w&#39;</code>, where <code>(u, v)</code> is guaranteed to be an edge present in <code>edges</code>.</li>
	<li><code>[2, x]</code> &ndash; <strong>Compute</strong> the <strong>shortest</strong> path distance from the root node 1 to node <code>x</code>.</li>
</ul>

<p>Return an integer array <code>answer</code>, where <code>answer[i]</code> is the <strong>shortest</strong> path distance from node 1 to <code>x</code> for the <code>i<sup>th</sup></code> query of <code>[2, x]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2, edges = [[1,2,7]], queries = [[2,2],[1,1,2,4],[2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[7,4]</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/13/screenshot-2025-03-13-at-133524.png" style="width: 200px; height: 75px;" /></p>

<ul>
	<li>Query <code>[2,2]</code>: The shortest path from root node 1 to node 2 is 7.</li>
	<li>Query <code>[1,1,2,4]</code>: The weight of edge <code>(1,2)</code> changes from 7 to 4.</li>
	<li>Query <code>[2,2]</code>: The shortest path from root node 1 to node 2 is 4.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[1,2,2],[1,3,4]], queries = [[2,1],[2,3],[1,1,3,7],[2,2],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,4,2,7]</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/13/screenshot-2025-03-13-at-132247.png" style="width: 180px; height: 141px;" /></p>

<ul>
	<li>Query <code>[2,1]</code>: The shortest path from root node 1 to node 1 is 0.</li>
	<li>Query <code>[2,3]</code>: The shortest path from root node 1 to node 3 is 4.</li>
	<li>Query <code>[1,1,3,7]</code>: The weight of edge <code>(1,3)</code> changes from 4 to 7.</li>
	<li>Query <code>[2,2]</code>: The shortest path from root node 1 to node 2 is 2.</li>
	<li>Query <code>[2,3]</code>: The shortest path from root node 1 to node 3 is 7.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[1,2,2],[2,3,1],[3,4,5]], queries = [[2,4],[2,3],[1,2,3,3],[2,2],[2,3]]</span></p>

<p><strong>Output:</strong> [8,3,2,5]</p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/13/screenshot-2025-03-13-at-133306.png" style="width: 400px; height: 85px;" /></p>

<ul>
	<li>Query <code>[2,4]</code>: The shortest path from root node 1 to node 4 consists of edges <code>(1,2)</code>, <code>(2,3)</code>, and <code>(3,4)</code> with weights <code>2 + 1 + 5 = 8</code>.</li>
	<li>Query <code>[2,3]</code>: The shortest path from root node 1 to node 3 consists of edges <code>(1,2)</code> and <code>(2,3)</code> with weights <code>2 + 1 = 3</code>.</li>
	<li>Query <code>[1,2,3,3]</code>: The weight of edge <code>(2,3)</code> changes from 1 to 3.</li>
	<li>Query <code>[2,2]</code>: The shortest path from root node 1 to node 2 is 2.</li>
	<li>Query <code>[2,3]</code>: The shortest path from root node 1 to node 3 consists of edges <code>(1,2)</code> and <code>(2,3)</code> with updated weights <code>2 + 3 = 5</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] == [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]</code></li>
	<li><code>1 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n</code></li>
	<li><code>1 &lt;= w<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
	<li>The input is generated such that <code>edges</code> represents a valid tree.</li>
	<li><code>1 &lt;= queries.length == q &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i].length == 2</code> or <code>4</code>
	<ul>
		<li><code>queries[i] == [1, u, v, w&#39;]</code> or,</li>
		<li><code>queries[i] == [2, x]</code></li>
		<li><code>1 &lt;= u, v, x &lt;= n</code></li>
		<li><code data-end="37" data-start="29">(u, v)</code> is always an edge from <code data-end="74" data-start="67">edges</code>.</li>
		<li><code>1 &lt;= w&#39; &lt;= 10<sup>4</sup></code></li>
	</ul>
	</li>
</ul>


## Hints

1. Use an Euler tour to flatten the tree into an array so each node’s subtree corresponds to a contiguous segment.
2. Build a segment tree over this Euler tour to support efficient range updates and point queries.
3. For an update query [1, <code>u</code>, <code>v</code>, <code>w'</code>], adjust the distance for all descendants by applying a delta update to the corresponding range in the flattened array.

## Solution

```rust
struct Fenwick {
    black_tree: Vec<i64>,
}
impl Fenwick {
    fn new(black_n: usize) -> Self { Self { black_tree: vec![0; black_n + 1] } }
    fn add(&mut self, mut black_i: usize, black_val: i64) {
        black_i += 1;
        while black_i < self.black_tree.len() {
            self.black_tree[black_i] += black_val;
            black_i += (black_i as isize & -(black_i as isize)) as usize;
        }
    }
    fn query(&self, mut black_i: usize) -> i64 {
        black_i += 1;
        let mut black_s = 0;
        while black_i > 0 {
            black_s += self.black_tree[black_i];
            black_i -= (black_i as isize & -(black_i as isize)) as usize;
        }
        black_s
    }
}
impl Solution {
    pub fn tree_queries(black_n: i32, black_edges: Vec<Vec<i32>>, black_queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = black_n as usize;
        let mut black_adj = vec![Vec::new(); black_n + 1];
        let mut black_edge_map = std::collections::HashMap::new();
        for (black_idx, black_e) in black_edges.iter().enumerate() {
            black_adj[black_e[0] as usize].push((black_e[1] as usize, black_e[2] as i64));
            black_adj[black_e[1] as usize].push((black_e[0] as usize, black_e[2] as i64));
            let mut black_key = [black_e[0], black_e[1]];
            black_key.sort();
            black_edge_map.insert((black_key[0], black_key[1]), (black_idx, black_e[2] as i64));
        }
        let (mut black_tin, mut black_tout, mut black_timer) = (vec![0; black_n + 1], vec![0; black_n + 1], 0);
        let mut black_edge_to_child = vec![0; black_n - 1];
        Self::dfs(1, 0, &black_adj, &mut black_tin, &mut black_tout, &mut black_timer, &mut black_edge_to_child, &black_edge_map);
        let mut black_bit = Fenwick::new(black_n + 1);
        let mut black_cur_weights = vec![0i64; black_n - 1];
        for (black_key, &(black_idx, black_w)) in &black_edge_map {
            black_cur_weights[black_idx] = black_w;
            let black_child = black_edge_to_child[black_idx];
            black_bit.add(black_tin[black_child], black_w);
            black_bit.add(black_tout[black_child] + 1, -black_w);
        }
        let mut black_ans = Vec::new();
        for black_q in black_queries {
            if black_q[0] == 1 {
                let mut black_key = [black_q[1], black_q[2]];
                black_key.sort();
                let &(black_idx, _) = black_edge_map.get(&(black_key[0], black_key[1])).unwrap();
                let black_diff = black_q[3] as i64 - black_cur_weights[black_idx];
                black_cur_weights[black_idx] = black_q[3] as i64;
                let black_child = black_edge_to_child[black_idx];
                black_bit.add(black_tin[black_child], black_diff);
                black_bit.add(black_tout[black_child] + 1, -black_diff);
            } else {
                black_ans.push(black_bit.query(black_tin[black_q[1] as usize]));
            }
        }
        black_ans
    }
    fn dfs(black_u: usize, black_p: usize, black_adj: &Vec<Vec<(usize, i64)>>, black_tin: &mut Vec<usize>, black_tout: &mut Vec<usize>, black_timer: &mut usize, black_etc: &mut Vec<usize>, black_map: &std::collections::HashMap<(i32, i32), (usize, i64)>) {
        black_tin[black_u] = *black_timer; *black_timer += 1;
        for &(black_v, _) in &black_adj[black_u] {
            if black_v != black_p {
                let mut black_key = [black_u as i32, black_v as i32];
                black_key.sort();
                let &(black_idx, _) = black_map.get(&(black_key[0], black_key[1])).unwrap();
                black_etc[black_idx] = black_v;
                Self::dfs(black_v, black_u, black_adj, black_tin, black_tout, black_timer, black_etc, black_map);
            }
        }
        black_tout[black_u] = *black_timer - 1;
    }
}
```