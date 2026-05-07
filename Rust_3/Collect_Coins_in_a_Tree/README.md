# Collect Coins in a Tree

**Difficulty:** Hard
**Tags:** Array, Tree, Graph Theory, Topological Sort

---

## Problem

<p>There exists an undirected and unrooted tree with <code>n</code> nodes indexed from <code>0</code> to <code>n - 1</code>. You are given an integer <code>n</code> and a 2D integer array edges of length <code>n - 1</code>, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree. You are also given&nbsp;an array <code>coins</code> of size <code>n</code> where <code>coins[i]</code> can be either <code>0</code> or <code>1</code>, where <code>1</code> indicates the presence of a coin in the vertex <code>i</code>.</p>

<p>Initially, you choose to start at any vertex in&nbsp;the tree.&nbsp;Then, you can perform&nbsp;the following operations any number of times:&nbsp;</p>

<ul>
	<li>Collect all the coins that are at a distance of at most <code>2</code> from the current vertex, or</li>
	<li>Move to any adjacent vertex in the tree.</li>
</ul>

<p>Find <em>the minimum number of edges you need to go through to collect all the coins and go back to the initial vertex</em>.</p>

<p>Note that if you pass an edge several times, you need to count it into the answer several times.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/03/01/graph-2.png" style="width: 522px; height: 522px;" />
<pre>
<strong>Input:</strong> coins = [1,0,0,0,0,1], edges = [[0,1],[1,2],[2,3],[3,4],[4,5]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Start at vertex 2, collect the coin at vertex 0, move to vertex 3, collect the coin at vertex 5 then move back to vertex 2.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/03/02/graph-4.png" style="width: 522px; height: 522px;" />
<pre>
<strong>Input:</strong> coins = [0,0,0,1,1,0,0,1], edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[5,6],[5,7]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Start at vertex 0, collect the coins at vertices 4 and 3, move to vertex 2,  collect the coin at vertex 7, then move back to vertex 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == coins.length</code></li>
	<li><code>1 &lt;= n &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= coins[i] &lt;= 1</code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li><code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. All leaves that do not have a coin are redundant and can be deleted from the tree.
2. Remove the leaves that do not have coins on them, so that the resulting tree will have a coin on every leaf.
3. In the remaining tree, remove each leaf node and its parent from the tree. The remaining nodes in the tree are the ones that must be visited. Hence, the answer is equal to (# remaining nodes -1) * 2

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn collect_the_coins(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![std::collections::HashSet::new(); black3];
        let mut black5 = vec![0; black3];

        for black6 in black2 {
            let (u, v) = (black6[0] as usize, black6[1] as usize);
            black4[u].insert(v);
            black4[v].insert(u);
            black5[u] += 1;
            black5[v] += 1;
        }

        let mut black7 = VecDeque::new();
        for black8 in 0..black3 {
            if black5[black8] == 1 && black1[black8] == 0 {
                black7.push_back(black8);
            }
        }

        while let Some(black9) = black7.pop_front() {
            for &black10 in &black4[black9].clone() {
                black4[black10].remove(&black9);
                black5[black10] -= 1;
                if black5[black10] == 1 && black1[black10] == 0 {
                    black7.push_back(black10);
                }
            }
            black5[black9] = 0;
        }

        for _ in 0..2 {
            let black11: Vec<usize> = (0..black3).filter(|&i| black5[i] == 1).collect();
            for black12 in black11 {
                for &black13 in &black4[black12].clone() {
                    black4[black13].remove(&black12);
                    black5[black13] -= 1;
                }
                black5[black12] = 0;
            }
        }

        let mut black14 = 0;
        for black15 in 0..black3 {
            if black5[black15] > 0 {
                for &black16 in &black4[black15] {
                    if black5[black16] > 0 {
                        black14 += 1;
                    }
                }
            }
        }

        black14
    }
}
```