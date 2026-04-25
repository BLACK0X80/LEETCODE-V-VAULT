# Critical Connections in a Network

**Difficulty:** Hard
**Tags:** Depth-First Search, Graph Theory, Biconnected Component

---

## Problem

<p>There are <code>n</code> servers numbered from <code>0</code> to <code>n - 1</code> connected by undirected server-to-server <code>connections</code> forming a network where <code>connections[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> represents a connection between servers <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. Any server can reach other servers directly or indirectly through the network.</p>

<p>A <em>critical connection</em> is a connection that, if removed, will make some servers unable to reach some other server.</p>

<p>Return all critical connections in the network in any order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/09/03/1537_ex1_2.png" style="width: 198px; height: 248px;" />
<pre>
<strong>Input:</strong> n = 4, connections = [[0,1],[1,2],[2,0],[1,3]]
<strong>Output:</strong> [[1,3]]
<strong>Explanation:</strong> [[3,1]] is also accepted.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2, connections = [[0,1]]
<strong>Output:</strong> [[0,1]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>n - 1 &lt;= connections.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li>There are no repeated connections.</li>
</ul>


## Hints

1. Use Tarjan's algorithm.

## Solution

```rust
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut black_adj = vec![vec![]; n];
        for c in &connections {
            black_adj[c[0] as usize].push(c[1] as usize);
            black_adj[c[1] as usize].push(c[0] as usize);
        }

        let mut black_disc = vec![usize::MAX; n];
        let mut black_low = vec![0usize; n];
        let mut black_ans: Vec<Vec<i32>> = vec![];
        let mut black_timer = 0usize;

        let mut black_stack: Vec<(usize, usize, usize)> = vec![(0, usize::MAX, 0)];
        let mut black_iter = vec![0usize; n];

        while let Some(&(u, parent, _)) = black_stack.last() {
            if black_disc[u] == usize::MAX {
                black_disc[u] = black_timer;
                black_low[u] = black_timer;
                black_timer += 1;
            }

            let mut black_pushed = false;
            while black_iter[u] < black_adj[u].len() {
                let v = black_adj[u][black_iter[u]];
                black_iter[u] += 1;
                if v == parent { continue; }
                if black_disc[v] == usize::MAX {
                    black_stack.push((v, u, black_iter[u] - 1));
                    black_pushed = true;
                    break;
                } else {
                    black_low[u] = black_low[u].min(black_disc[v]);
                }
            }

            if !black_pushed {
                black_stack.pop();
                if let Some(&(pu, _, _)) = black_stack.last() {
                    if black_low[u] > black_disc[pu] {
                        black_ans.push(vec![pu as i32, u as i32]);
                    }
                    black_low[pu] = black_low[pu].min(black_low[u]);
                }
            }
        }

        black_ans
    }
}
```