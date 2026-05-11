# Count Visited Nodes in a Directed Graph

**Difficulty:** Hard
**Tags:** Dynamic Programming, Depth-First Search, Graph Theory, Topological Sort, Memoization

---

## Problem

<p>There is a <strong>directed</strong> graph consisting of <code>n</code> nodes numbered from <code>0</code> to <code>n - 1</code> and <code>n</code> directed edges.</p>

<p>You are given a <strong>0-indexed</strong> array <code>edges</code> where <code>edges[i]</code> indicates that there is an edge from node <code>i</code> to node <code>edges[i]</code>.</p>

<p>Consider the following process on the graph:</p>

<ul>
	<li>You start from a node <code>x</code> and keep visiting other nodes through edges until you reach a node that you have already visited before on this <strong>same</strong> process.</li>
</ul>

<p>Return <em>an array </em><code>answer</code><em> where </em><code>answer[i]</code><em> is the number of <strong>different</strong> nodes that you will visit if you perform the process starting from node </em><code>i</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/31/graaphdrawio-1.png" />
<pre>
<strong>Input:</strong> edges = [1,2,0,0]
<strong>Output:</strong> [3,3,3,4]
<strong>Explanation:</strong> We perform the process starting from each node in the following way:
- Starting from node 0, we visit the nodes 0 -&gt; 1 -&gt; 2 -&gt; 0. The number of different nodes we visit is 3.
- Starting from node 1, we visit the nodes 1 -&gt; 2 -&gt; 0 -&gt; 1. The number of different nodes we visit is 3.
- Starting from node 2, we visit the nodes 2 -&gt; 0 -&gt; 1 -&gt; 2. The number of different nodes we visit is 3.
- Starting from node 3, we visit the nodes 3 -&gt; 0 -&gt; 1 -&gt; 2 -&gt; 0. The number of different nodes we visit is 4.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/31/graaph2drawio.png" style="width: 191px; height: 251px;" />
<pre>
<strong>Input:</strong> edges = [1,2,3,4,0]
<strong>Output:</strong> [5,5,5,5,5]
<strong>Explanation:</strong> Starting from any node we can visit every node in the graph in the process.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == edges.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= edges[i] &lt;= n - 1</code></li>
	<li><code>edges[i] != i</code></li>
</ul>


## Hints

1. Consider if the graph was only one cycle, what will be the answer for each node?
2. The actual graph will always consist of at least one cycle and some other nodes.
3. Calculate the answer for nodes in cycles the same way as in hint 1. How do you calculate the answer for the remaining nodes?

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn count_visited_nodes(black1: Vec<i32>) -> Vec<i32> {
        let black2 = black1.len();
        let mut black3 = vec![0; black2];
        let mut black4 = vec![0; black2];
        
        for &black5 in &black1 {
            black4[black5 as usize] += 1;
        }

        let mut black6 = VecDeque::new();
        for black7 in 0..black2 {
            if black4[black7] == 0 {
                black6.push_back(black7);
            }
        }

        let mut black8 = vec![false; black2];
        while let Some(black9) = black6.pop_front() {
            black8[black9] = true;
            let black10 = black1[black9] as usize;
            black4[black10] -= 1;
            if black4[black10] == 0 {
                black6.push_back(black10);
            }
        }

        for black11 in 0..black2 {
            if !black8[black11] && black3[black11] == 0 {
                let mut black12 = Vec::new();
                let mut black13 = black11;
                while black3[black13] == 0 {
                    black3[black13] = -1; 
                    black12.push(black13);
                    black13 = black1[black13] as usize;
                }
                let black14 = black12.len() as i32;
                for black15 in black12 {
                    black3[black15] = black14;
                }
            }
        }

        fn black_dfs(black16: usize, black17: &Vec<i32>, black18: &mut Vec<i32>) -> i32 {
            if black18[black16] != 0 {
                return black18[black16];
            }
            black18[black16] = black_dfs(black17[black16] as usize, black17, black18) + 1;
            black18[black16]
        }

        for black19 in 0..black2 {
            if black8[black19] {
                black_dfs(black19, &black1, &mut black3);
            }
        }

        black3
    }
}
```