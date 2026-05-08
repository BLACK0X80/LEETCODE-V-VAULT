# Longest Special Path II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Tree, Depth-First Search, Prefix Sum

---

## Problem

<p>You are given an undirected tree rooted at node <code>0</code>, with <code>n</code> nodes numbered from <code>0</code> to <code>n - 1</code>. This is represented by a 2D array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, length<sub>i</sub>]</code> indicates an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> with length <code>length<sub>i</sub></code>. You are also given an integer array <code>nums</code>, where <code>nums[i]</code> represents the value at node <code>i</code>.</p>

<p>A <strong>special path</strong> is defined as a <strong>downward</strong> path from an ancestor node to a descendant node in which all node values are <strong>distinct</strong>, except for <strong>at most</strong> one value that may appear twice.</p>

<p>Return an array <code data-stringify-type="code">result</code> of size 2, where <code>result[0]</code> is the <b data-stringify-type="bold">length</b> of the <strong>longest</strong> special path, and <code>result[1]</code> is the <b data-stringify-type="bold">minimum</b> number of nodes in all <i data-stringify-type="italic">possible</i> <strong>longest</strong> special paths.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[0,1,1],[1,2,3],[1,3,1],[2,4,6],[4,7,2],[3,5,2],[3,6,5],[6,8,3]], nums = [1,1,0,3,1,2,1,1,0]</span></p>

<p><strong>Output:</strong> <span class="example-io">[9,3]</span></p>

<p><strong>Explanation:</strong></p>

<p>In the image below, nodes are colored by their corresponding values in <code>nums</code>.</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/02/18/e1.png" style="width: 190px; height: 270px;" /></p>

<p>The longest special paths are <code>1 -&gt; 2 -&gt; 4</code> and <code>1 -&gt; 3 -&gt; 6 -&gt; 8</code>, both having a length of 9. The minimum number of nodes across all longest special paths is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[1,0,3],[0,2,4],[0,3,5]], nums = [1,1,0,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">[5,2]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/02/18/e2.png" style="width: 150px; height: 110px;" /></p>

<p>The longest path is <code>0 -&gt; 3</code> consisting of 2 nodes with a length of 5.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 5 * 10<sup><span style="font-size: 10.8333px;">4</span></sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i].length == 3</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; n</code></li>
	<li><code>1 &lt;= length<sub>i</sub> &lt;= 10<sup>3</sup></code></li>
	<li><code>nums.length == n</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 5 * 10<sup>4</sup></code></li>
	<li>The input is generated such that <code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. Maintain a special path (from root to current node) dynamically.
2. Also, maintain the positions of each value on the path so we can adjust the start point of the path.
3. Use prefix sum to calculate the path length.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_special_path(black_edges: Vec<Vec<i32>>, black_nums: Vec<i32>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_graph = vec![Vec::new(); black_n];
        for black_e in black_edges {
            black_graph[black_e[0] as usize].push((black_e[1] as usize, black_e[2]));
            black_graph[black_e[1] as usize].push((black_e[0] as usize, black_e[2]));
        }

        let mut black_res = vec![0, 1];
        let mut black_cur_path = Vec::new();
        let mut black_last = HashMap::new();

        Self::dfs(
            0,
            0,
            usize::MAX,
            &vec![0, 0],
            &black_graph,
            &black_nums,
            &mut black_cur_path,
            &mut black_last,
            &mut black_res,
        );

        black_res
    }

    fn dfs(
        black_node: usize,
        black_curr_cost: i32,
        black_prev: usize,
        black_seen: &Vec<usize>,
        black_graph: &Vec<Vec<(usize, i32)>>,
        black_nums: &Vec<i32>,
        black_cur_path: &mut Vec<i32>,
        black_last: &mut HashMap<i32, i32>,
        black_res: &mut Vec<i32>,
    ) {
        let black_color = black_nums[black_node];
        let black_last_color_idx = *black_last.get(&black_color).unwrap_or(&-1);
        
        black_last.insert(black_color, black_cur_path.len() as i32);
        black_cur_path.push(black_curr_cost);

        let black_start = black_seen[0];
        let black_path_len = black_curr_cost - black_cur_path[black_start];
        let black_node_count = (black_cur_path.len() - black_start) as i32;

        if black_path_len > black_res[0] {
            black_res[0] = black_path_len;
            black_res[1] = black_node_count;
        } else if black_path_len == black_res[0] {
            black_res[1] = black_res[1].min(black_node_count);
        }

        for &(black_next_node, black_next_cost) in &black_graph[black_node] {
            if black_next_node == black_prev {
                continue;
            }

            let mut black_next_seen = black_seen.clone();
            let black_next_color = black_nums[black_next_node];
            let black_found_idx = *black_last.get(&black_next_color).unwrap_or(&-1);

            if black_found_idx != -1 && (black_start as i32) <= black_found_idx {
                black_next_seen.push((black_found_idx + 1) as usize);
                black_next_seen.sort_unstable();
                if black_next_seen.len() > 2 {
                    black_next_seen.remove(0);
                }
            }

            Self::dfs(
                black_next_node,
                black_curr_cost + black_next_cost,
                black_node,
                &black_next_seen,
                black_graph,
                black_nums,
                black_cur_path,
                black_last,
                black_res,
            );
        }

        black_last.insert(black_color, black_last_color_idx);
        black_cur_path.pop();
    }
}
```