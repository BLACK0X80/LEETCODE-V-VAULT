# Paint House III

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>There is a row of <code>m</code> houses in a small city, each house must be painted with one of the <code>n</code> colors (labeled from <code>1</code> to <code>n</code>), some houses that have been painted last summer should not be painted again.</p>

<p>A neighborhood is a maximal group of continuous houses that are painted with the same color.</p>

<ul>
	<li>For example: <code>houses = [1,2,2,3,3,2,1,1]</code> contains <code>5</code> neighborhoods <code>[{1}, {2,2}, {3,3}, {2}, {1,1}]</code>.</li>
</ul>

<p>Given an array <code>houses</code>, an <code>m x n</code> matrix <code>cost</code> and an integer <code>target</code> where:</p>

<ul>
	<li><code>houses[i]</code>: is the color of the house <code>i</code>, and <code>0</code> if the house is not painted yet.</li>
	<li><code>cost[i][j]</code>: is the cost of paint the house <code>i</code> with the color <code>j + 1</code>.</li>
</ul>

<p>Return <em>the minimum cost of painting all the remaining houses in such a way that there are exactly</em> <code>target</code> <em>neighborhoods</em>. If it is not possible, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> houses = [0,0,0,0,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
<strong>Output:</strong> 9
<strong>Explanation:</strong> Paint houses of this way [1,2,2,1,1]
This array contains target = 3 neighborhoods, [{1}, {2,2}, {1,1}].
Cost of paint all houses (1 + 1 + 1 + 1 + 5) = 9.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> houses = [0,2,1,2,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
<strong>Output:</strong> 11
<strong>Explanation:</strong> Some houses are already painted, Paint the houses of this way [2,2,1,2,2]
This array contains target = 3 neighborhoods, [{2,2}, {1}, {2,2}]. 
Cost of paint the first and last house (10 + 1) = 11.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> houses = [3,1,2,3], cost = [[1,1,1],[1,1,1],[1,1,1],[1,1,1]], m = 4, n = 3, target = 3
<strong>Output:</strong> -1
<strong>Explanation:</strong> Houses are already painted with a total of 4 neighborhoods [{3},{1},{2},{3}] different of target = 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == houses.length == cost.length</code></li>
	<li><code>n == cost[i].length</code></li>
	<li><code>1 &lt;= m &lt;= 100</code></li>
	<li><code>1 &lt;= n &lt;= 20</code></li>
	<li><code>1 &lt;= target &lt;= m</code></li>
	<li><code>0 &lt;= houses[i] &lt;= n</code></li>
	<li><code>1 &lt;= cost[i][j] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use Dynamic programming.
2. Define dp[i][j][k] as the minimum cost where we have k neighborhoods in the first i houses and the i-th house is painted with the color j.

## Solution

```rust
impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (black_m, black_n, black_t) = (m as usize, n as usize, target as usize);
        let mut black_dp = vec![vec![vec![1_000_000_000; black_t + 1]; black_n + 1]; black_m];
        if houses[0] == 0 {
            for black_c in 1..=black_n { black_dp[0][black_c][1] = cost[0][black_c - 1]; }
        } else { black_dp[0][houses[0] as usize][1] = 0; }
        for black_i in 1..black_m {
            for black_k in 1..=black_t {
                for black_c in 1..=black_n {
                    if houses[black_i] != 0 && houses[black_i] as usize != black_c { continue; }
                    let black_cost = if houses[black_i] == 0 { cost[black_i][black_c - 1] } else { 0 };
                    let mut black_prev = 1_000_000_000;
                    for black_p in 1..=black_n {
                        let black_pk = if black_p == black_c { black_k } else { black_k - 1 };
                        if black_pk > 0 { black_prev = black_prev.min(black_dp[black_i - 1][black_p][black_pk]); }
                    }
                    black_dp[black_i][black_c][black_k] = black_prev + black_cost;
                }
            }
        }
        let mut black_ans = 1_000_000_000;
        for black_c in 1..=black_n { black_ans = black_ans.min(black_dp[black_m - 1][black_c][black_t]); }
        if black_ans >= 1_000_000_000 { -1 } else { black_ans }
    }
}
```