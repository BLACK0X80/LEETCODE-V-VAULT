# Maximum Genetic Difference Query

**Difficulty:** Hard
**Tags:** Array, Hash Table, Bit Manipulation, Depth-First Search, Trie

---

## Problem

<p>There is a rooted tree consisting of <code>n</code> nodes numbered <code>0</code> to <code>n - 1</code>. Each node&#39;s number denotes its <strong>unique genetic value</strong> (i.e. the genetic value of node <code>x</code> is <code>x</code>). The <strong>genetic difference</strong> between two genetic values is defined as the <strong>bitwise-</strong><strong>XOR</strong> of their values. You are given the integer array <code>parents</code>, where <code>parents[i]</code> is the parent for node <code>i</code>. If node <code>x</code> is the <strong>root</strong> of the tree, then <code>parents[x] == -1</code>.</p>

<p>You are also given the array <code>queries</code> where <code>queries[i] = [node<sub>i</sub>, val<sub>i</sub>]</code>. For each query <code>i</code>, find the <strong>maximum genetic difference</strong> between <code>val<sub>i</sub></code> and <code>p<sub>i</sub></code>, where <code>p<sub>i</sub></code> is the genetic value of any node that is on the path between <code>node<sub>i</sub></code> and the root (including <code>node<sub>i</sub></code> and the root). More formally, you want to maximize <code>val<sub>i</sub> XOR p<sub>i</sub></code>.</p>

<p>Return <em>an array </em><code>ans</code><em> where </em><code>ans[i]</code><em> is the answer to the </em><code>i<sup>th</sup></code><em> query</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/c1.png" style="width: 118px; height: 163px;" />
<pre>
<strong>Input:</strong> parents = [-1,0,1,1], queries = [[0,2],[3,2],[2,5]]
<strong>Output:</strong> [2,3,7]
<strong>Explanation: </strong>The queries are processed as follows:
- [0,2]: The node with the maximum genetic difference is 0, with a difference of 2 XOR 0 = 2.
- [3,2]: The node with the maximum genetic difference is 1, with a difference of 2 XOR 1 = 3.
- [2,5]: The node with the maximum genetic difference is 2, with a difference of 5 XOR 2 = 7.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/c2.png" style="width: 256px; height: 221px;" />
<pre>
<strong>Input:</strong> parents = [3,7,-1,2,0,7,0,2], queries = [[4,6],[1,15],[0,5]]
<strong>Output:</strong> [6,14,7]
<strong>Explanation: </strong>The queries are processed as follows:
- [4,6]: The node with the maximum genetic difference is 0, with a difference of 6 XOR 0 = 6.
- [1,15]: The node with the maximum genetic difference is 1, with a difference of 15 XOR 1 = 14.
- [0,5]: The node with the maximum genetic difference is 2, with a difference of 5 XOR 2 = 7.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= parents.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= parents[i] &lt;= parents.length - 1</code> for every node <code>i</code> that is <strong>not</strong> the root.</li>
	<li><code>parents[root] == -1</code></li>
	<li><code>1 &lt;= queries.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= node<sub>i</sub> &lt;= parents.length - 1</code></li>
	<li><code>0 &lt;= val<sub>i</sub> &lt;= 2 * 10<sup>5</sup></code></li>
</ul>


## Hints

1. How can we use a trie to store all the XOR values in the path from a node to the root?
2. How can we dynamically add the XOR values with a DFS search?

## Solution

```rust
impl Solution {
    pub fn max_genetic_difference(black_p: Vec<i32>, black_q: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_p.len();
        let mut black_adj = vec![vec![]; black_n];
        let mut black_root = 0;
        for (i, &p) in black_p.iter().enumerate() {
            if p == -1 { black_root = i; } else { black_adj[p as usize].push(i); }
        }
        let mut black_qs = vec![vec![]; black_n];
        for (i, q) in black_q.iter().enumerate() { black_qs[q[0] as usize].push((q[1], i)); }
        
        let mut black_ans = vec![0; black_q.len()];
        let mut black_trie = BlackTrie { nodes: vec![[0, 0, 0]] };
        Self::black_dfs(black_root, &black_adj, &black_qs, &mut black_trie, &mut black_ans);
        black_ans
    }

    fn black_dfs(u: usize, adj: &Vec<Vec<usize>>, qs: &Vec<Vec<(i32, usize)>>, t: &mut BlackTrie, res: &mut Vec<i32>) {
        t.black_upd(u as i32, 1);
        for &(v, i) in &qs[u] { res[i] = t.black_get(v); }
        for &v in &adj[u] { Self::black_dfs(v, adj, qs, t, res); }
        t.black_upd(u as i32, -1);
    }
}

struct BlackTrie { nodes: Vec<[i32; 3]> }
impl BlackTrie {
    fn black_upd(&mut self, v: i32, d: i32) {
        let mut curr = 0;
        for i in (0..18).rev() {
            let b = ((v >> i) & 1) as usize;
            if self.nodes[curr][b] == 0 {
                self.nodes[curr][b] = self.nodes.len() as i32;
                self.nodes.push([0, 0, 0]);
            }
            curr = self.nodes[curr][b] as usize;
            self.nodes[curr][2] += d;
        }
    }
    fn black_get(&self, v: i32) -> i32 {
        let (mut curr, mut res) = (0, 0);
        for i in (0..18).rev() {
            let b = ((v >> i) & 1) as usize;
            let target = 1 - b;
            let next = self.nodes[curr][target];
            if next != 0 && self.nodes[next as usize][2] > 0 {
                res |= 1 << i;
                curr = next as usize;
            } else {
                curr = self.nodes[curr][b] as usize;
            }
        }
        res
    }
}
```