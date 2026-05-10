# Construct 2D Grid Matching Graph Layout

**Difficulty:** Hard
**Tags:** Array, Hash Table, Graph Theory, Matrix

---

## Problem

<p>You are given a 2D integer array <code>edges</code> representing an <strong>undirected</strong> graph having <code>n</code> nodes, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> denotes an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</p>

<p>Construct a 2D grid that satisfies these conditions:</p>

<ul>
	<li>The grid contains <strong>all nodes</strong> from <code>0</code> to <code>n - 1</code> in its cells, with each node appearing exactly <strong>once</strong>.</li>
	<li>Two nodes should be in adjacent grid cells (<strong>horizontally</strong> or <strong>vertically</strong>) <strong>if and only if</strong> there is an edge between them in <code>edges</code>.</li>
</ul>

<p>It is guaranteed that <code>edges</code> can form a 2D grid that satisfies the conditions.</p>

<p>Return a 2D integer array satisfying the conditions above. If there are multiple solutions, return <em>any</em> of them.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[0,1],[0,2],[1,3],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[3,1],[2,0]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/11/screenshot-from-2024-08-11-14-07-59.png" style="width: 133px; height: 92px;" /></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, edges = [[0,1],[1,3],[2,3],[2,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[4,2,3,1,0]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2024/08/11/screenshot-from-2024-08-11-14-06-02.png" style="width: 325px; height: 50px;" /></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 9, edges = [[0,1],[0,4],[0,5],[1,7],[2,3],[2,4],[2,5],[3,6],[4,6],[4,7],[6,8],[7,8]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[8,6,3],[7,4,2],[1,0,5]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/11/screenshot-from-2024-08-11-14-06-38.png" style="width: 198px; height: 133px;" /></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= edges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub> &lt; v<sub>i</sub> &lt; n</code></li>
	<li>All the edges are distinct.</li>
	<li>The input is generated such that <code>edges</code> can form a 2D grid that satisfies the conditions.</li>
</ul>


## Hints

1. Observe the indegrees of the nodes.
2. The case where there are two nodes with an indegree of 1, and all the others have an indegree of 2 can be handled separately.
3. The nodes with the smallest degrees are the corners.
4. You can simulate the grid creation process using BFS or a similar approach after making some observations on the indegrees.

## Solution

```rust
impl Solution {
    pub fn construct_grid_layout(black_n_i32: i32, black_edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let black_n = black_n_i32 as usize;
        let mut black_g = vec![Vec::new(); black_n];
        for black_e in black_edges {
            let (black_u, black_v) = (black_e[0] as usize, black_e[1] as usize);
            black_g[black_u].push(black_v);
            black_g[black_v].push(black_u);
        }

        let mut black_min_d = usize::MAX;
        for black_v in &black_g {
            black_min_d = black_min_d.min(black_v.len());
        }

        let mut black_corners = Vec::new();
        for black_i in 0..black_n {
            if black_g[black_i].len() == black_min_d {
                black_corners.push(black_i);
            }
        }

        let black_dis = |black_start: usize| -> Vec<i32> {
            let mut black_d = vec![-1; black_n];
            black_d[black_start] = 1;
            let mut black_queue = std::collections::VecDeque::new();
            black_queue.push_back(black_start);
            while let Some(black_u) = black_queue.pop_front() {
                for &black_v in &black_g[black_u] {
                    if black_d[black_v] == -1 {
                        black_d[black_v] = black_d[black_u] + 1;
                        black_queue.push_back(black_v);
                    }
                }
            }
            black_d
        };

        let black_d1 = black_dis(black_corners[0]);
        let mut black_corner_distances = Vec::new();
        for &black_v in &black_corners {
            black_corner_distances.push(black_d1[black_v]);
        }
        black_corner_distances.sort_unstable();
        
        let black_c_val = black_corner_distances[1];
        let mut black_second_corner = 0;
        for &black_v in &black_corners {
            if black_d1[black_v] == black_c_val {
                black_second_corner = black_v;
                break;
            }
        }

        let black_d2 = black_dis(black_second_corner);
        let mut black_res_indices: Vec<usize> = (0..black_n).collect();
        black_res_indices.sort_by_key(|&black_i| (black_d1[black_i] + black_d2[black_i], black_d1[black_i]));

        let black_cols = black_c_val as usize;
        let mut black_final_grid = Vec::new();
        for black_chunk in black_res_indices.chunks(black_cols) {
            black_final_grid.push(black_chunk.iter().map(|&black_x| black_x as i32).collect());
        }

        black_final_grid
    }
}
```