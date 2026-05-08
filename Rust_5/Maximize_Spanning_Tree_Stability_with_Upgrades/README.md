# Maximize Spanning Tree Stability with Upgrades

**Difficulty:** Hard
**Tags:** Binary Search, Greedy, Union-Find, Graph Theory, Minimum Spanning Tree

---

## Problem

<p>You are given an integer <code>n</code>, representing <code>n</code> nodes numbered from 0 to <code>n - 1</code> and a list of <code>edges</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, s<sub>i</sub>, must<sub>i</sub>]</code>:</p>

<ul>
	<li><code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> indicates an undirected edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</li>
	<li><code>s<sub>i</sub></code> is the strength of the edge.</li>
	<li><code>must<sub>i</sub></code> is an integer (0 or 1). If <code>must<sub>i</sub> == 1</code>, the edge <strong>must</strong> be included in the<strong> </strong><strong>spanning tree</strong>. These edges <strong>cannot</strong> be <strong>upgraded</strong>.</li>
</ul>

<p>You are also given an integer <code>k</code>, the <strong>maximum</strong> number of upgrades you can perform. Each upgrade <strong>doubles</strong> the strength of an edge, and each eligible edge (with <code>must<sub>i</sub> == 0</code>) can be upgraded <strong>at most</strong> once.</p>

<p>The <strong>stability</strong> of a spanning tree is defined as the <strong>minimum</strong> strength score among all edges included in it.</p>

<p>Return the <strong>maximum</strong> possible stability of any valid spanning tree. If it is impossible to connect all nodes, return <code>-1</code>.</p>

<p><strong>Note</strong>: A <strong>spanning tree</strong> of a graph with <code>n</code> nodes is a subset of the edges that connects all nodes together (i.e. the graph is <strong>connected</strong>) <em>without</em> forming any cycles, and uses <strong>exactly</strong> <code>n - 1</code> edges.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1,2,1],[1,2,3,0]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Edge <code>[0,1]</code> with strength = 2 must be included in the spanning tree.</li>
	<li>Edge <code>[1,2]</code> is optional and can be upgraded from 3 to 6 using one upgrade.</li>
	<li>The resulting spanning tree includes these two edges with strengths 2 and 6.</li>
	<li>The minimum strength in the spanning tree is 2, which is the maximum possible stability.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1,4,0],[1,2,3,0],[0,2,1,0]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Since all edges are optional and up to <code>k = 2</code> upgrades are allowed.</li>
	<li>Upgrade edges <code>[0,1]</code> from 4 to 8 and <code>[1,2]</code> from 3 to 6.</li>
	<li>The resulting spanning tree includes these two edges with strengths 8 and 6.</li>
	<li>The minimum strength in the tree is 6, which is the maximum possible stability.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1,1,1],[1,2,1,1],[2,0,1,1]], k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>All edges are mandatory and form a cycle, which violates the spanning tree property of acyclicity. Thus, the answer is -1.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= edges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, s<sub>i</sub>, must<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; n</code></li>
	<li><code>u<sub>i</sub> != v<sub>i</sub></code></li>
	<li><code>1 &lt;= s<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
	<li><code>must<sub>i</sub></code> is either <code>0</code> or <code>1</code>.</li>
	<li><code>0 &lt;= k &lt;= n</code></li>
	<li>There are no duplicate edges.</li>
</ul>


## Hints

1. Sort the <code>edges</code> array in descending order of weights.
2. Try using binary search on <code>ans</code>.
3. Implement a <code>chk</code> function which first adds all the edges with <code>must = 1</code>, and then adds the edges with <code>must = 0</code>, using any remaining upgrades greedily.
4. Use a <code>DSU</code> with path compression and union by size/rank to maintain connected components.
5. Don't forget the case where you cannot form an MST because more than one component remains after processing all edges.

## Solution

```rust
impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut black_low = 1;
        let mut black_high = 200001;
        let mut black_ans = -1;

        let mut black_p_init = (0..n as usize).collect::<Vec<_>>();
        for black_e in &edges {
            if black_e[3] == 1 {
                let black_r1 = Self::find(black_e[0] as usize, &mut black_p_init);
                let black_r2 = Self::find(black_e[1] as usize, &mut black_p_init);
                if black_r1 == black_r2 { return -1; }
                black_p_init[black_r1] = black_r2;
            }
        }

        while black_low < black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::check(n, &edges, k, black_mid) {
                black_ans = black_mid;
                black_low = black_mid + 1;
            } else {
                black_high = black_mid;
            }
        }
        black_ans
    }

    fn check(black_n: i32, black_edges: &Vec<Vec<i32>>, black_k: i32, black_target: i32) -> bool {
        let mut black_parent = (0..black_n as usize).collect::<Vec<_>>();
        let mut black_count = 0;
        let mut black_up = 0;

        for black_e in black_edges {
            if black_e[3] == 1 {
                if black_e[2] < black_target { return false; }
                let black_r1 = Self::find(black_e[0] as usize, &mut black_parent);
                let black_r2 = Self::find(black_e[1] as usize, &mut black_parent);
                black_parent[black_r1] = black_r2;
                black_count += 1;
            }
        }

        for black_e in black_edges {
            if black_e[3] == 0 && black_e[2] >= black_target {
                let black_r1 = Self::find(black_e[0] as usize, &mut black_parent);
                let black_r2 = Self::find(black_e[1] as usize, &mut black_parent);
                if black_r1 != black_r2 {
                    black_parent[black_r1] = black_r2;
                    black_count += 1;
                }
            }
        }

        for black_e in black_edges {
            if black_e[3] == 0 && black_e[2] < black_target && black_e[2] * 2 >= black_target {
                if black_up < black_k {
                    let black_r1 = Self::find(black_e[0] as usize, &mut black_parent);
                    let black_r2 = Self::find(black_e[1] as usize, &mut black_parent);
                    if black_r1 != black_r2 {
                        black_parent[black_r1] = black_r2;
                        black_count += 1;
                        black_up += 1;
                    }
                }
            }
        }

        black_count == black_n - 1
    }

    fn find(black_i: usize, black_p: &mut Vec<usize>) -> usize {
        if black_p[black_i] == black_i { return black_i; }
        black_p[black_i] = Self::find(black_p[black_i], black_p);
        black_p[black_i]
    }
}
```