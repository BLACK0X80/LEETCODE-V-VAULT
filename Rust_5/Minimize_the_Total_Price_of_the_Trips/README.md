# Minimize the Total Price of the Trips

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Tree, Depth-First Search, Graph Theory

---

## Problem

<p>There exists an undirected and unrooted tree with <code>n</code> nodes indexed from <code>0</code> to <code>n - 1</code>. You are given the integer <code>n</code> and a 2D integer array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree.</p>

<p>Each node has an associated price. You are given an integer array <code>price</code>, where <code>price[i]</code> is the price of the <code>i<sup>th</sup></code> node.</p>

<p>The <strong>price sum</strong> of a given path is the sum of the prices of all nodes lying on that path.</p>

<p>Additionally, you are given a 2D integer array <code>trips</code>, where <code>trips[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> indicates that you start the <code>i<sup>th</sup></code> trip from the node <code>start<sub>i</sub></code> and travel to the node <code>end<sub>i</sub></code> by any path you like.</p>

<p>Before performing your first trip, you can choose some <strong>non-adjacent</strong> nodes and halve the prices.</p>

<p>Return <em>the minimum total price sum to perform all the given trips</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/03/16/diagram2.png" style="width: 541px; height: 181px;" />
<pre>
<strong>Input:</strong> n = 4, edges = [[0,1],[1,2],[1,3]], price = [2,2,10,6], trips = [[0,3],[2,1],[2,3]]
<strong>Output:</strong> 23
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 2. The first part shows the initial tree and the second part shows the tree after choosing nodes 0, 2, and 3, and making their price half.
For the 1<sup>st</sup> trip, we choose path [0,1,3]. The price sum of that path is 1 + 2 + 3 = 6.
For the 2<sup>nd</sup> trip, we choose path [2,1]. The price sum of that path is 2 + 5 = 7.
For the 3<sup>rd</sup> trip, we choose path [2,1,3]. The price sum of that path is 5 + 2 + 3 = 10.
The total price sum of all trips is 6 + 7 + 10 = 23.
It can be proven, that 23 is the minimum answer that we can achieve.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/03/16/diagram3.png" style="width: 456px; height: 111px;" />
<pre>
<strong>Input:</strong> n = 2, edges = [[0,1]], price = [2,2], trips = [[0,0]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 0. The first part shows the initial tree and the second part shows the tree after choosing node 0, and making its price half.
For the 1<sup>st</sup> trip, we choose path [0]. The price sum of that path is 1.
The total price sum of all trips is 1. It can be proven, that 1 is the minimum answer that we can achieve.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 50</code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>edges</code> represents a valid tree.</li>
	<li><code>price.length == n</code></li>
	<li><code>price[i]</code> is an even integer.</li>
	<li><code>1 &lt;= price[i] &lt;= 1000</code></li>
	<li><code>1 &lt;= trips.length &lt;= 100</code></li>
	<li><code>0 &lt;= start<sub>i</sub>, end<sub>i</sub>&nbsp;&lt;= n - 1</code></li>
</ul>


## Hints

1. The final answer is the price[i] * freq[i], where freq[i] is the number of times node i was visited during the trip, and price[i] is the final price.
2. To find freq[i] we will use dfs or bfs for each trip and update every node on the path start and end.
3. Finally, to find the final price[i] we will use dynamic programming on the tree. Let dp(v, 0/1) denote the minimum total price with the node v’s price being halved or not.

## Solution

```rust
impl Solution {
    pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n];
        for black_e in edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_count = vec![0; black_n];
        for black_t in trips {
            let (black_start, black_end) = (black_t[0] as usize, black_t[1] as usize);
            let mut black_path = vec![];
            fn black_find(u: usize, p: usize, t: usize, adj: &Vec<Vec<usize>>, path: &mut Vec<usize>) -> bool {
                path.push(u);
                if u == t { return true; }
                for &v in &adj[u] {
                    if v != p && black_find(v, u, t, adj, path) { return true; }
                }
                path.pop();
                false
            }
            black_find(black_start, black_n, black_end, &black_adj, &mut black_path);
            for black_node in black_path { black_count[black_node] += 1; }
        }

        fn black_solve(u: usize, p: usize, adj: &Vec<Vec<usize>>, count: &Vec<i32>, price: &Vec<i32>) -> (i32, i32) {
            let mut black_h = (price[u] / 2) * count[u];
            let mut black_f = price[u] * count[u];
            for &v in &adj[u] {
                if v == p { continue; }
                let (black_vh, black_vf) = black_solve(v, u, adj, count, price);
                black_h += black_vf;
                black_f += black_vh.min(black_vf);
            }
            (black_h, black_f)
        }

        let (black_res_h, black_res_f) = black_solve(0, black_n, &black_adj, &black_count, &price);
        black_res_h.min(black_res_f)
    }
}
```