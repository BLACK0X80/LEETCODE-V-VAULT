# Longest Special Path

**Difficulty:** Hard
**Tags:** Array, Hash Table, Tree, Depth-First Search, Prefix Sum

---

## Problem

<p>You are given an undirected tree rooted at node <code>0</code> with <code>n</code> nodes numbered from <code>0</code> to <code>n - 1</code>, represented by a 2D array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, length<sub>i</sub>]</code> indicates an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> with length <code>length<sub>i</sub></code>. You are also given an integer array <code>nums</code>, where <code>nums[i]</code> represents the value at node <code>i</code>.</p>

<p>A <b data-stringify-type="bold">special path</b> is defined as a <b data-stringify-type="bold">downward</b> path from an ancestor node to a descendant node such that all the values of the nodes in that path are <b data-stringify-type="bold">unique</b>.</p>

<p><strong>Note</strong> that a path may start and end at the same node.</p>

<p>Return an array <code data-stringify-type="code">result</code> of size 2, where <code>result[0]</code> is the <b data-stringify-type="bold">length</b> of the <strong>longest</strong> special path, and <code>result[1]</code> is the <b data-stringify-type="bold">minimum</b> number of nodes in all <i data-stringify-type="italic">possible</i> <strong>longest</strong> special paths.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[0,1,2],[1,2,3],[1,3,5],[1,4,4],[2,5,6]], nums = [2,1,2,1,3,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">[6,2]</span></p>

<p><strong>Explanation:</strong></p>

<h4>In the image below, nodes are colored by their corresponding values in <code>nums</code></h4>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/11/02/tree3.jpeg" style="width: 250px; height: 350px;" /></p>

<p>The longest special paths are <code>2 -&gt; 5</code> and <code>0 -&gt; 1 -&gt; 4</code>, both having a length of 6. The minimum number of nodes across all longest special paths is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[1,0,8]], nums = [2,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,1]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/11/02/tree4.jpeg" style="width: 190px; height: 75px;" /></p>

<p>The longest special paths are <code>0</code> and <code>1</code>, both having a length of 0. The minimum number of nodes across all longest special paths is 1.</p>
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

1. Use DFS to traverse the tree and maintain the current path length from the root (starting at 0) to the current node.
2. Use prefix sums to calculate the longest path ending at the current node with all unique values.

## Solution

```rust
impl Solution {
    pub fn longest_special_path(black_edges: Vec<Vec<i32>>, black_nums: Vec<i32>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_adj = vec![Vec::new(); black_n];
        for black_e in black_edges {
            let black_u = black_e[0] as usize;
            let black_v = black_e[1] as usize;
            let black_w = black_e[2];
            black_adj[black_u].push((black_v, black_w));
            black_adj[black_v].push((black_u, black_w));
        }

        let mut black_ans_len = 0;
        let mut black_ans_nodes = 1;
        let mut black_last_pos = vec![-1i32; 50001];
        let mut black_dist_stack = vec![0];

        fn black_dfs(
            black_u: usize,
            black_p: usize,
            black_curr_dist: i32,
            black_max_top: i32,
            black_adj: &Vec<Vec<(usize, i32)>>,
            black_nums: &Vec<i32>,
            black_last_pos: &mut Vec<i32>,
            black_dist_stack: &mut Vec<i32>,
            black_ans_len: &mut i32,
            black_ans_nodes: &mut i32,
        ) {
            let black_val = black_nums[black_u] as usize;
            let black_prev_idx = black_last_pos[black_val];
            let black_new_top = black_max_top.max(black_prev_idx);

            let black_top_idx = (black_new_top + 1) as usize;
            let black_d = black_curr_dist - black_dist_stack[black_top_idx];
            let black_cnt = (black_dist_stack.len() - black_top_idx) as i32;

            if black_d > *black_ans_len {
                *black_ans_len = black_d;
                *black_ans_nodes = black_cnt;
            } else if black_d == *black_ans_len {
                if black_cnt < *black_ans_nodes {
                    *black_ans_nodes = black_cnt;
                }
            }

            let black_old_pos = black_last_pos[black_val];
            black_last_pos[black_val] = (black_dist_stack.len() - 1) as i32;

            for &(black_v, black_w) in &black_adj[black_u] {
                if black_v != black_p {
                    black_dist_stack.push(black_curr_dist + black_w);
                    black_dfs(
                        black_v,
                        black_u,
                        black_curr_dist + black_w,
                        black_new_top,
                        black_adj,
                        black_nums,
                        black_last_pos,
                        black_dist_stack,
                        black_ans_len,
                        black_ans_nodes,
                    );
                    black_dist_stack.pop();
                }
            }
            black_last_pos[black_val] = black_old_pos;
        }

        black_dfs(
            0,
            0,
            0,
            -1,
            &black_adj,
            &black_nums,
            &mut black_last_pos,
            &mut black_dist_stack,
            &mut black_ans_len,
            &mut black_ans_nodes,
        );

        vec![black_ans_len, black_ans_nodes]
    }
}
```