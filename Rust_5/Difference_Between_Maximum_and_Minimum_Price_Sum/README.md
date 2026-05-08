# Difference Between Maximum and Minimum Price Sum

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Tree, Depth-First Search

---

## Problem

<p>There exists an undirected and initially unrooted tree with <code>n</code> nodes indexed from <code>0</code> to <code>n - 1</code>. You are given the integer <code>n</code> and a 2D integer array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree.</p>

<p>Each node has an associated price. You are given an integer array <code>price</code>, where <code>price[i]</code> is the price of the <code>i<sup>th</sup></code> node.</p>

<p>The <strong>price sum</strong> of a given path is the sum of the prices of all nodes lying on that path.</p>

<p>The tree can be rooted at any node <code>root</code> of your choice. The incurred <strong>cost</strong> after choosing <code>root</code> is the difference between the maximum and minimum <strong>price sum</strong> amongst all paths starting at <code>root</code>.</p>

<p>Return <em>the <strong>maximum</strong> possible <strong>cost</strong></em> <em>amongst all possible root choices</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/12/01/example14.png" style="width: 556px; height: 231px;" />
<pre>
<strong>Input:</strong> n = 6, edges = [[0,1],[1,2],[1,3],[3,4],[3,5]], price = [9,8,7,6,10,5]
<strong>Output:</strong> 24
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 2. The first part (colored in red) shows the path with the maximum price sum. The second part (colored in blue) shows the path with the minimum price sum.
- The first path contains nodes [2,1,3,4]: the prices are [7,8,6,10], and the sum of the prices is 31.
- The second path contains the node [2] with the price [7].
The difference between the maximum and minimum price sum is 24. It can be proved that 24 is the maximum cost.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/11/24/p1_example2.png" style="width: 352px; height: 184px;" />
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[1,2]], price = [1,1,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 0. The first part (colored in red) shows the path with the maximum price sum. The second part (colored in blue) shows the path with the minimum price sum.
- The first path contains nodes [0,1,2]: the prices are [1,1,1], and the sum of the prices is 3.
- The second path contains node [0] with a price [1].
The difference between the maximum and minimum price sum is 2. It can be proved that 2 is the maximum cost.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>edges</code> represents a valid tree.</li>
	<li><code>price.length == n</code></li>
	<li><code>1 &lt;= price[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. The minimum price sum is always the price of a rooted node.
2. Let’s root the tree at vertex 0 and find the answer from this perspective.
3. In the optimal answer maximum price is the sum of the prices of nodes on the path from “u” to “v” where either “u” or “v” is the parent of the second one or neither is a parent of the second one.
4. The first case is easy to find. For the second case, notice that in the optimal path, “u” and “v” are both leaves. Then we can use dynamic programming to find such a path.
5. Let DP(v,1) denote “the maximum price sum from node v to leaf, where v is a parent of that leaf” and let DP(v,0) denote “the maximum price sum from node v to leaf, where v is a parent of that leaf - price[leaf]”. Then the answer is maximum of DP(u,0) + DP(v,1) + price[parent] where u, v are directly connected to vertex “parent”.

## Solution

```rust
impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut black1 = vec![vec![]; n];
        for e in edges {
            black1[e[0] as usize].push(e[1] as usize);
            black1[e[1] as usize].push(e[0] as usize);
        }
        let mut black2 = 0i64;
        fn dfs(u: usize, p: usize, g: &Vec<Vec<usize>>, pr: &Vec<i32>, res: &mut i64) -> (i64, i64) {
            let (mut black3, mut black4) = (pr[u] as i64, 0i64);
            for &v in &g[u] {
                if v != p {
                    let (s1, s2) = dfs(v, u, g, pr, res);
                    *res = (*res).max((black3 + s2).max(black4 + s1));
                    black3 = black3.max(s1 + pr[u] as i64);
                    black4 = black4.max(s2 + pr[u] as i64);
                }
            }
            (black3, black4)
        }
        dfs(0, 0, &black1, &price, &mut black2);
        black2
    }
}
```