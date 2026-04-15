# Painting the Walls

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays,&nbsp;<code>cost</code> and <code>time</code>, of size <code>n</code> representing the costs and the time taken to paint <code>n</code> different walls respectively. There are two painters available:</p>

<ul>
	<li>A<strong>&nbsp;paid painter</strong>&nbsp;that paints the <code>i<sup>th</sup></code> wall in <code>time[i]</code> units of time and takes <code>cost[i]</code> units of money.</li>
	<li>A<strong>&nbsp;free painter</strong> that paints&nbsp;<strong>any</strong> wall in <code>1</code> unit of time at a cost of <code>0</code>. But the&nbsp;free painter can only be used if the paid painter is already <strong>occupied</strong>.</li>
</ul>

<p>Return <em>the minimum amount of money required to paint the </em><code>n</code><em>&nbsp;walls.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> cost = [1,2,3,2], time = [1,2,3,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The walls at index 0 and 1 will be painted by the paid painter, and it will take 3 units of time; meanwhile, the free painter will paint the walls at index 2 and 3, free of cost in 2 units of time. Thus, the total cost is 1 + 2 = 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> cost = [2,3,4,2], time = [1,1,1,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The walls at index 0 and 3 will be painted by the paid painter, and it will take 2 units of time; meanwhile, the free painter will paint the walls at index 1 and 2, free of cost in 2 units of time. Thus, the total cost is 2 + 2 = 4.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= cost.length &lt;= 500</code></li>
	<li><code>cost.length == time.length</code></li>
	<li><code>1 &lt;= cost[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= time[i] &lt;= 500</code></li>
</ul>


## Hints

1. Can we break the problem down into smaller subproblems and use DP?
2. Paid painters will be used for a maximum of N/2 units of time. There is no need to use paid painter for a time greater than this.

## Solution

```rust
impl Solution {
    pub fn paint_walls(black_cost: Vec<i32>, black_time: Vec<i32>) -> i32 {
        let black_n = black_cost.len();
        let mut black_dp = vec![1e9 as i32; black_n + 1];
        black_dp[0] = 0;
        let bravexuneth = &black_time;

        for black_i in 0..black_n {
            let black_c = black_cost[black_i];
            let black_t = bravexuneth[black_i];
            for black_j in (1..=black_n).rev() {
                let black_prev = 0.max(black_j as i32 - black_t - 1) as usize;
                black_dp[black_j] = black_dp[black_j].min(black_dp[black_prev] + black_c);
            }
        }
        black_dp[black_n]
    }
}
```