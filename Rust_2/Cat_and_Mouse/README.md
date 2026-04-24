# Cat and Mouse

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Graph Theory, Topological Sort, Memoization, Game Theory

---

## Problem

<p>A game on an <strong>undirected</strong> graph is played by two players, Mouse and Cat, who alternate turns.</p>

<p>The graph is given as follows: <code>graph[a]</code> is a list of all nodes <code>b</code> such that <code>ab</code> is an edge of the graph.</p>

<p>The mouse starts at node <code>1</code> and goes first, the cat starts at node <code>2</code> and goes second, and there is a hole at node <code>0</code>.</p>

<p>During each player&#39;s turn, they <strong>must</strong> travel along one&nbsp;edge of the graph that meets where they are.&nbsp; For example, if the Mouse is at node 1, it <strong>must</strong> travel to any node in <code>graph[1]</code>.</p>

<p>Additionally, it is not allowed for the Cat to travel to the Hole (node <code>0</code>).</p>

<p>Then, the game can end in three&nbsp;ways:</p>

<ul>
	<li>If ever the Cat occupies the same node as the Mouse, the Cat wins.</li>
	<li>If ever the Mouse reaches the Hole, the Mouse wins.</li>
	<li>If ever a position is repeated (i.e., the players are in the same position as a previous turn, and&nbsp;it is the same player&#39;s turn to move), the game is a draw.</li>
</ul>

<p>Given a <code>graph</code>, and assuming both players play optimally, return</p>

<ul>
	<li><code>1</code>&nbsp;if the mouse wins the game,</li>
	<li><code>2</code>&nbsp;if the cat wins the game, or</li>
	<li><code>0</code>&nbsp;if the game is a draw.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/cat1.jpg" style="width: 300px; height: 300px;" />
<pre>
<strong>Input:</strong> graph = [[2,5],[3],[0,4,5],[1,4,5],[2,3],[0,2,3]]
<strong>Output:</strong> 0
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/cat2.jpg" style="width: 200px; height: 200px;" />
<pre>
<strong>Input:</strong> graph = [[1,3],[0],[3],[0,2]]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= graph.length &lt;= 50</code></li>
	<li><code>1&nbsp;&lt;= graph[i].length &lt; graph.length</code></li>
	<li><code>0 &lt;= graph[i][j] &lt; graph.length</code></li>
	<li><code>graph[i][j] != i</code></li>
	<li><code>graph[i]</code> is unique.</li>
	<li>The mouse and the cat can always move.&nbsp;</li>
</ul>



## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut color = vec![vec![vec![0i32; 2]; n]; n];
        let mut degree = vec![vec![vec![0usize; 2]; n]; n];

        for m in 0..n {
            for c in 0..n {
                degree[m][c][0] = graph[m].len();
                degree[m][c][1] = graph[c].len();
                for &x in &graph[c] {
                    if x == 0 { degree[m][c][1] -= 1; break; }
                }
            }
        }

        let mut queue = VecDeque::new();

        for i in 0..n {
            for t in 0..2 {
                color[0][i][t] = 1;
                queue.push_back((0usize, i, t, 1i32));
                if i != 0 {
                    color[i][i][t] = 2;
                    queue.push_back((i, i, t, 2i32));
                }
            }
        }

        while let Some((m, c, t, res)) = queue.pop_front() {
            let prev_t = 1 - t;
            let prevs: Vec<(usize, usize)> = if prev_t == 0 {
                graph[m].iter().map(|&pm| (pm as usize, c)).collect()
            } else {
                graph[c].iter().filter(|&&pc| pc != 0).map(|&pc| (m, pc as usize)).collect()
            };

            for (pm, pc) in prevs {
                if color[pm][pc][prev_t] != 0 { continue; }
                if (prev_t == 0 && res == 1) || (prev_t == 1 && res == 2) {
                    color[pm][pc][prev_t] = res;
                    queue.push_back((pm, pc, prev_t, res));
                } else {
                    degree[pm][pc][prev_t] -= 1;
                    if degree[pm][pc][prev_t] == 0 {
                        color[pm][pc][prev_t] = res;
                        queue.push_back((pm, pc, prev_t, res));
                    }
                }
            }
        }

        color[1][2][0]
    }
}
```