# Minimum Edge Toggles on a Tree

**Difficulty:** Hard
**Tags:** Tree, Depth-First Search, Graph Theory, Topological Sort, Sorting

---

## Problem

<p>You are given an <strong>undirected tree</strong> with <code>n</code> nodes, numbered from 0 to <code>n - 1</code>. It is represented by a 2D integer array <code>edges</code>​​​​​​​ of length <code>n - 1</code>, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree.</p>

<p>You are also given two <strong>binary</strong> strings <code>start</code> and <code>target</code> of length <code>n</code>. For each node <code>x</code>, <code>start[x]</code> is its initial color and <code>target[x]</code> is its desired color.</p>

<p>In one operation, you may pick an edge with index <code>i</code> and <strong>toggle </strong>both of its endpoints. That is, if the edge is <code>[u, v]</code>, then the colors of nodes <code>u</code> and <code>v</code> <strong>each</strong> flip from <code>&#39;0&#39;</code> to <code>&#39;1&#39;</code> or from <code>&#39;1&#39;</code> to <code>&#39;0&#39;</code>.</p>

<p>Return an array of edge indices whose operations transform <code>start</code> into <code>target</code>. Among all valid sequences with <strong>minimum possible length</strong>, return the edge indices in <strong>increasing</strong>​​​​​​​ order.</p>

<p>If it is impossible to transform <code>start</code> into <code>target</code>, return an array containing a single element equal to -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2025/12/18/example1.png" style="width: 271px; height: 51px;" />​​​​​​​</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1],[1,2]], start = &quot;010&quot;, target = &quot;100&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">[0]</span></p>

<p><strong>Explanation:</strong></p>

<p>Toggle edge with index 0, which flips nodes 0 and 1.<br />
​​​​​​​The string changes from <code>&quot;010&quot;</code> to <code>&quot;100&quot;</code>, matching the target.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<p><strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2025/12/18/example2.png" style="width: 411px; height: 208px;" /></strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 7, edges = [[0,1],[1,2],[2,3],[3,4],[3,5],[1,6]], start = &quot;0011000&quot;, target = &quot;0010001&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2,5]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Toggle edge with index 1, which flips nodes 1 and 2.</li>
	<li>Toggle edge with index 2, which flips nodes 2 and 3.</li>
	<li>Toggle edge with index 5, which flips nodes 1 and 6.</li>
</ul>

<p>After these operations, the resulting string becomes <code>&quot;0010001&quot;</code>, which matches the target.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<p><strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2025/12/18/example3.png" style="width: 161px; height: 51px;" />​​​​​​​</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2, edges = [[0,1]], start = &quot;00&quot;, target = &quot;01&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">[-1]</span></p>

<p><strong>Explanation:</strong></p>

<p>There is no sequence of edge toggles that transforms <code>&quot;00&quot;</code> into <code>&quot;01&quot;</code>. Therefore, we return <code>[-1]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == start.length == target.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>start[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
	<li><code>target[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
	<li>The input is generated such that <code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. Use a depth-first search (DFS).
2. Root the tree at any node. Traverse the tree, keeping track of flips applied from the parent to the subtree.
3. After processing a child subtree, determine whether the current node still needs to be flipped to match the target. If a flip is needed, record the corresponding edge.
4. Collect all flipped edges; if the transformation is impossible, return <code>[-1]</code>.

## Solution

```rust
impl Solution {
    pub fn minimum_flips(black1: i32, black2: Vec<Vec<i32>>, black3: String, black4: String) -> Vec<i32> {
        let black5 = black1 as usize;
        let mut black6 = vec![vec![]; black5];
        let black7: Vec<u8> = black3.bytes().zip(black4.bytes()).map(|(s, t)| (s ^ t)).collect();
        if black7.iter().fold(0, |acc, &x| acc ^ x) != 0 { return vec![-1]; }

        for (black8, black9) in black2.iter().enumerate() {
            black6[black9[0] as usize].push((black9[1] as usize, black8 as i32));
            black6[black9[1] as usize].push((black9[0] as usize, black8 as i32));
        }

        let mut black10 = vec![0; black5];
        let mut black11 = Vec::new();
        let mut black12 = vec![0; black5];
        fn black_dfs(u: usize, p: usize, black6: &Vec<Vec<(usize, i32)>>, black7: &Vec<u8>, black10: &mut Vec<u8>, black11: &mut Vec<i32>) {
            let mut black13 = black7[u];
            for &(v, id) in &black6[u] {
                if v == p { continue; }
                black_dfs(v, u, black6, black7, black10, black11);
                if black10[v] == 1 {
                    black11.push(id);
                    black13 ^= 1;
                }
            }
            black10[u] = black13;
        }
        black_dfs(0, 0, &black6, &black7, &mut black10, &mut black11);
        black11.sort_unstable();
        black11
    }
}
```