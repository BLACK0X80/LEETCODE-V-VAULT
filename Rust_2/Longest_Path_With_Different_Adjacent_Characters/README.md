# Longest Path With Different Adjacent Characters

**Difficulty:** Hard
**Tags:** Array, String, Tree, Depth-First Search, Graph Theory, Topological Sort

---

## Problem

<p>You are given a <strong>tree</strong> (i.e. a connected, undirected graph that has no cycles) <strong>rooted</strong> at node <code>0</code> consisting of <code>n</code> nodes numbered from <code>0</code> to <code>n - 1</code>. The tree is represented by a <strong>0-indexed</strong> array <code>parent</code> of size <code>n</code>, where <code>parent[i]</code> is the parent of node <code>i</code>. Since node <code>0</code> is the root, <code>parent[0] == -1</code>.</p>

<p>You are also given a string <code>s</code> of length <code>n</code>, where <code>s[i]</code> is the character assigned to node <code>i</code>.</p>

<p>Return <em>the length of the <strong>longest path</strong> in the tree such that no pair of <strong>adjacent</strong> nodes on the path have the same character assigned to them.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/25/testingdrawio.png" style="width: 201px; height: 241px;" />
<pre>
<strong>Input:</strong> parent = [-1,0,0,1,1,2], s = &quot;abacbe&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest path where each two adjacent nodes have different characters in the tree is the path: 0 -&gt; 1 -&gt; 3. The length of this path is 3, so 3 is returned.
It can be proven that there is no longer path that satisfies the conditions. 
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/25/graph2drawio.png" style="width: 201px; height: 221px;" />
<pre>
<strong>Input:</strong> parent = [-1,0,0,0], s = &quot;aabc&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest path where each two adjacent nodes have different characters is the path: 2 -&gt; 0 -&gt; 3. The length of this path is 3, so 3 is returned.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == parent.length == s.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= parent[i] &lt;= n - 1</code> for all <code>i &gt;= 1</code></li>
	<li><code>parent[0] == -1</code></li>
	<li><code>parent</code> represents a valid tree.</li>
	<li><code>s</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. Do a DFS from the root. At each node, calculate the longest path we can make from two branches of that subtree.
2. To do that, we need to find the length of the longest path from each of the node’s children.

## Solution

```rust
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let b = s.as_bytes();
        let mut children = vec![vec![]; n];
        for i in 1..n { children[parent[i] as usize].push(i); }
        let mut ans = 1;
        fn dfs(u: usize, children: &Vec<Vec<usize>>, b: &[u8], ans: &mut i32) -> i32 {
            let mut top2 = [0i32; 2];
            for &v in &children[u] {
                let d = dfs(v, children, b, ans);
                if b[v] != b[u] {
                    if d > top2[0] { top2[1]=top2[0]; top2[0]=d; }
                    else if d > top2[1] { top2[1]=d; }
                }
            }
            *ans = (*ans).max(top2[0]+top2[1]+1);
            top2[0]+1
        }
        dfs(0, &children, b, &mut ans);
        ans
    }
}
```