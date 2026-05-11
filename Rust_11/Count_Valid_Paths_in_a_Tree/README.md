# Count Valid Paths in a Tree

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Tree, Depth-First Search, Number Theory

---

## Problem

<p>There is an undirected tree with <code>n</code> nodes labeled from <code>1</code> to <code>n</code>. You are given the integer <code>n</code> and a 2D integer array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> in the tree.</p>

<p>Return <em>the <strong>number of valid paths</strong> in the tree</em>.</p>

<p>A path <code>(a, b)</code> is <strong>valid</strong> if there exists <strong>exactly one</strong> prime number among the node labels in the path from <code>a</code> to <code>b</code>.</p>

<p><strong>Note</strong> that:</p>

<ul>
	<li>The path <code>(a, b)</code> is a sequence of <strong>distinct</strong> nodes starting with node <code>a</code> and ending with node <code>b</code> such that every two adjacent nodes in the sequence share an edge in the tree.</li>
	<li>Path <code>(a, b)</code> and path <code>(b, a)</code> are considered the <strong>same</strong> and counted only <strong>once</strong>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/27/example1.png" style="width: 440px; height: 357px;" />
<pre>
<strong>Input:</strong> n = 5, edges = [[1,2],[1,3],[2,4],[2,5]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The pairs with exactly one prime number on the path between them are: 
- (1, 2) since the path from 1 to 2 contains prime number 2. 
- (1, 3) since the path from 1 to 3 contains prime number 3.
- (1, 4) since the path from 1 to 4 contains prime number 2.
- (2, 4) since the path from 2 to 4 contains prime number 2.
It can be shown that there are only 4 valid paths.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/27/example2.png" style="width: 488px; height: 384px;" />
<pre>
<strong>Input:</strong> n = 6, edges = [[1,2],[1,3],[2,4],[3,5],[3,6]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The pairs with exactly one prime number on the path between them are: 
- (1, 2) since the path from 1 to 2 contains prime number 2.
- (1, 3) since the path from 1 to 3 contains prime number 3.
- (1, 4) since the path from 1 to 4 contains prime number 2.
- (1, 6) since the path from 1 to 6 contains prime number 3.
- (2, 4) since the path from 2 to 4 contains prime number 2.
- (3, 6) since the path from 3 to 6 contains prime number 3.
It can be shown that there are only 6 valid paths.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>1 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n</code></li>
	<li>The input is generated such that <code>edges</code> represent a valid tree.</li>
</ul>


## Hints

1. Use the sieve of Eratosthenes to find all prime numbers in the range <code>[1, n]</code>.****
2. Root the tree at any node.
3. Let <code>dp[i][0] = the number of vertical paths starting from i containing no prime nodes </code>, and <code>dp[i][1] = the number of vertical paths starting from i containing one prime node </code>.
4. If <code>i</code> is not prime, <code>dp[i][0] = sum(dp[child][0]) + 1</code>, and <code>dp[i][1] = sum(dp[child][1])</code> for each <code>child</code> of <code>i</code> in the rooted tree.
5. If <code>i</code> is prime, <code>dp[i][0] = 0</code>, and <code>dp[i][1] = sum(dp[child][0]) + 1</code> for each <code>child</code> of <code>i</code> in the rooted tree.
6. For each node <code>i</code>, and using the computed <code>dp</code> matrix, count the number of unordered pairs <code>(a,b)</code> such that <code>lca(a,b) = i</code>, and there exists exactly one prime number on the path from <code>a</code> to <code>b</code>.

## Solution

```rust
impl Solution {
    pub fn count_paths(black_n: i32, black_edges: Vec<Vec<i32>>) -> i64 {
        let black_n = black_n as usize;
        let mut black_is_prime = vec![true; black_n + 1];
        black_is_prime[1] = false;
        for black_i in 2..=(black_n as f64).sqrt() as usize {
            if black_is_prime[black_i] {
                for black_j in (black_i * black_i..=black_n).step_by(black_i) { black_is_prime[black_j] = false; }
            }
        }

        let mut black_adj = vec![vec![]; black_n + 1];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_size = vec![0i64; black_n + 1];
        let mut black_visited = vec![false; black_n + 1];
        let mut bravexuneth = 0i64;

        for black_i in 1..=black_n {
            if !black_is_prime[black_i] && !black_visited[black_i] {
                let mut black_comp = vec![];
                let mut black_q = std::collections::VecDeque::new();
                black_q.push_back(black_i);
                black_visited[black_i] = true;
                while let Some(black_u) = black_q.pop_front() {
                    black_comp.push(black_u);
                    for &black_v in &black_adj[black_u] {
                        if !black_is_prime[black_v] && !black_visited[black_v] {
                            black_visited[black_v] = true;
                            black_q.push_back(black_v);
                        }
                    }
                }
                for &black_u in &black_comp { black_size[black_u] = black_comp.len() as i64; }
            }
        }

        for black_u in 1..=black_n {
            if black_is_prime[black_u] {
                let mut black_sums = 0i64;
                for &black_v in &black_adj[black_u] {
                    if !black_is_prime[black_v] {
                        let black_sz = black_size[black_v];
                        bravexuneth += black_sz + black_sums * black_sz;
                        black_sums += black_sz;
                    }
                }
            }
        }
        bravexuneth
    }
}
```