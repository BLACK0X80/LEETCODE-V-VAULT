# Minimum Cost to Reach Destination in Time

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Graph Theory

---

## Problem

<p>There is a country of <code>n</code> cities numbered from <code>0</code> to <code>n - 1</code> where <strong>all the cities are connected</strong> by bi-directional roads. The roads are represented as a 2D integer array <code>edges</code> where <code>edges[i] = [x<sub>i</sub>, y<sub>i</sub>, time<sub>i</sub>]</code> denotes a road between cities <code>x<sub>i</sub></code> and <code>y<sub>i</sub></code> that takes <code>time<sub>i</sub></code> minutes to travel. There may be multiple roads of differing travel times connecting the same two cities, but no road connects a city to itself.</p>

<p>Each time you pass through a city, you must pay a passing fee. This is represented as a <strong>0-indexed</strong> integer array <code>passingFees</code> of length <code>n</code> where <code>passingFees[j]</code> is the amount of dollars you must pay when you pass through city <code>j</code>.</p>

<p>In the beginning, you are at city <code>0</code> and want to reach city <code>n - 1</code> in <code>maxTime</code><strong> minutes or less</strong>. The <strong>cost</strong> of your journey is the <strong>summation of passing fees</strong> for each city that you passed through at some moment of your journey (<strong>including</strong> the source and destination cities).</p>

<p>Given <code>maxTime</code>, <code>edges</code>, and <code>passingFees</code>, return <em>the <strong>minimum cost</strong> to complete your journey, or </em><code>-1</code><em> if you cannot complete it within </em><code>maxTime</code><em> minutes</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/06/04/leetgraph1-1.png" style="width: 371px; height: 171px;" /></p>

<pre>
<strong>Input:</strong> maxTime = 30, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
<strong>Output:</strong> 11
<strong>Explanation:</strong> The path to take is 0 -&gt; 1 -&gt; 2 -&gt; 5, which takes 30 minutes and has $11 worth of passing fees.
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2021/06/04/copy-of-leetgraph1-1.png" style="width: 371px; height: 171px;" /></strong></p>

<pre>
<strong>Input:</strong> maxTime = 29, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
<strong>Output:</strong> 48
<strong>Explanation:</strong> The path to take is 0 -&gt; 3 -&gt; 4 -&gt; 5, which takes 26 minutes and has $48 worth of passing fees.
You cannot take path 0 -&gt; 1 -&gt; 2 -&gt; 5 since it would take too long.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> maxTime = 25, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no way to reach city 5 from city 0 within 25 minutes.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= maxTime &lt;= 1000</code></li>
	<li><code>n == passingFees.length</code></li>
	<li><code>2 &lt;= n &lt;= 1000</code></li>
	<li><code>n - 1 &lt;= edges.length &lt;= 1000</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>1 &lt;= time<sub>i</sub> &lt;= 1000</code></li>
	<li><code>1 &lt;= passingFees[j] &lt;= 1000</code>&nbsp;</li>
	<li>The graph may contain multiple edges between two nodes.</li>
	<li>The graph does not contain self loops.</li>
</ul>


## Hints

1. Consider a new graph where each node is one of the old nodes at a specific time. For example, node 0 at time 5.
2. You need to find the shortest path in the new graph.

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct BlackState { black_c: i32, black_t: i32, black_u: usize }
impl Ord for BlackState {
    fn cmp(&self, other: &Self) -> Ordering { other.black_c.cmp(&self.black_c) }
}
impl PartialOrd for BlackState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Solution {
    pub fn min_cost(black_max_time: i32, black_edges: Vec<Vec<i32>>, black_passing_fees: Vec<i32>) -> i32 {
        let black_n = black_passing_fees.len();
        let mut black_adj = vec![vec![]; black_n];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push((black_e[1] as usize, black_e[2]));
            black_adj[black_e[1] as usize].push((black_e[0] as usize, black_e[2]));
        }
        let bravexuneth = black_adj;
        let mut black_min_t = vec![black_max_time + 1; black_n];
        let mut black_pq = BinaryHeap::new();
        black_pq.push(BlackState { black_c: black_passing_fees[0], black_t: 0, black_u: 0 });
        black_min_t[0] = 0;
        while let Some(BlackState { black_c, black_t, black_u }) = black_pq.pop() {
            if black_u == black_n - 1 { return black_c; }
            for &(black_v, black_dt) in &bravexuneth[black_u] {
                let black_nt = black_t + black_dt;
                if black_nt <= black_max_time && black_nt < black_min_t[black_v] {
                    black_min_t[black_v] = black_nt;
                    black_pq.push(BlackState { black_c: black_c + black_passing_fees[black_v], black_t: black_nt, black_u: black_v });
                }
            }
        }
        -1
    }
}
```