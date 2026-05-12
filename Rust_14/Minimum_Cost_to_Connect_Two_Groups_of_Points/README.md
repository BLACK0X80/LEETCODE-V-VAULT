# Minimum Cost to Connect Two Groups of Points

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation, Matrix, Bitmask

---

## Problem

<p>You are given two groups of points where the first group has <code>size<sub>1</sub></code> points, the second group has <code>size<sub>2</sub></code> points, and <code>size<sub>1</sub> &gt;= size<sub>2</sub></code>.</p>

<p>The <code>cost</code> of the connection between any two points are given in an <code>size<sub>1</sub> x size<sub>2</sub></code> matrix where <code>cost[i][j]</code> is the cost of connecting point <code>i</code> of the first group and point <code>j</code> of the second group. The groups are connected if <strong>each point in both groups is connected to one or more points in the opposite group</strong>. In other words, each point in the first group must be connected to at least one point in the second group, and each point in the second group must be connected to at least one point in the first group.</p>

<p>Return <em>the minimum cost it takes to connect the two groups</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/ex1.jpg" style="width: 322px; height: 243px;" />
<pre>
<strong>Input:</strong> cost = [[15, 96], [36, 2]]
<strong>Output:</strong> 17
<strong>Explanation</strong>: The optimal way of connecting the groups is:
1--A
2--B
This results in a total cost of 17.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/ex2.jpg" style="width: 322px; height: 403px;" />
<pre>
<strong>Input:</strong> cost = [[1, 3, 5], [4, 1, 1], [1, 5, 3]]
<strong>Output:</strong> 4
<strong>Explanation</strong>: The optimal way of connecting the groups is:
1--A
2--B
2--C
3--A
This results in a total cost of 4.
Note that there are multiple points connected to point 2 in the first group and point A in the second group. This does not matter as there is no limit to the number of points that can be connected. We only care about the minimum total cost.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> cost = [[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]]
<strong>Output:</strong> 10
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>size<sub>1</sub> == cost.length</code></li>
	<li><code>size<sub>2</sub> == cost[i].length</code></li>
	<li><code>1 &lt;= size<sub>1</sub>, size<sub>2</sub> &lt;= 12</code></li>
	<li><code>size<sub>1</sub> &gt;= size<sub>2</sub></code></li>
	<li><code>0 &lt;= cost[i][j] &lt;= 100</code></li>
</ul>


## Hints

1. Each point on the left would either be connected to exactly point already connected to some left node, or a subset of the nodes on the right which are not connected to any node
2. Use dynamic programming with bitmasking, where the state will be (number of points assigned in first group, bitmask of points assigned in second group).

## Solution

```rust
impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let n = cost.len();
        let m = cost[0].len();
        let mut min_col = vec![i32::MAX; m];
        for j in 0..m {
            for i in 0..n {
                if cost[i][j] < min_col[j] { min_col[j] = cost[i][j]; }
            }
        }
        let mut dp = vec![i32::MAX; 1 << m];
        dp[0] = 0;
        for i in 0..n {
            let mut next_dp = vec![i32::MAX; 1 << m];
            for mask in 0..(1 << m) {
                if dp[mask] == i32::MAX { continue; }
                for j in 0..m {
                    let nmask = mask | (1 << j);
                    let val = dp[mask] + cost[i][j];
                    if val < next_dp[nmask] { next_dp[nmask] = val; }
                }
            }
            dp = next_dp;
        }
        let mut ans = i32::MAX;
        for mask in 0..(1 << m) {
            if dp[mask] == i32::MAX { continue; }
            let mut cur = dp[mask];
            for j in 0..m {
                if mask & (1 << j) == 0 { cur += min_col[j]; }
            }
            if cur < ans { ans = cur; }
        }
        ans
    }
}
```