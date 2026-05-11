# Allocate Mailboxes

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Sorting

---

## Problem

<p>Given the array <code>houses</code> where <code>houses[i]</code> is the location of the <code>i<sup>th</sup></code> house along a street and an integer <code>k</code>, allocate <code>k</code> mailboxes in the street.</p>

<p>Return <em>the <strong>minimum</strong> total distance between each house and its nearest mailbox</em>.</p>

<p>The test cases are generated so that the answer fits in a 32-bit integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/sample_11_1816.png" style="width: 454px; height: 154px;" />
<pre>
<strong>Input:</strong> houses = [1,4,8,10,20], k = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong> Allocate mailboxes in position 3, 9 and 20.
Minimum total distance from each houses to nearest mailboxes is |3-1| + |4-3| + |9-8| + |10-9| + |20-20| = 5 
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/sample_2_1816.png" style="width: 433px; height: 154px;" />
<pre>
<strong>Input:</strong> houses = [2,3,5,12,18], k = 2
<strong>Output:</strong> 9
<strong>Explanation:</strong> Allocate mailboxes in position 3 and 14.
Minimum total distance from each houses to nearest mailboxes is |2-3| + |3-3| + |5-3| + |12-14| + |18-14| = 9.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= houses.length &lt;= 100</code></li>
	<li><code>1 &lt;= houses[i] &lt;= 10<sup>4</sup></code></li>
	<li>All the integers of <code>houses</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. If k =1, the minimum distance is obtained allocating the mailbox in the median of the array houses.
2. Generalize this idea, using dynamic programming allocating k mailboxes.

## Solution

```rust
impl Solution {
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        houses.sort();
        let n = houses.len();
        let k = k as usize;
        let mut cost = vec![vec![0; n]; n];
        for i in 0..n {
            for j in i..n {
                let mid = (i + j) / 2;
                for l in i..=j { cost[i][j] += (houses[l] - houses[mid]).abs(); }
            }
        }
        let mut dp = vec![vec![i32::MAX; n]; k];
        for j in 0..n { dp[0][j] = cost[0][j]; }
        for i in 1..k {
            for j in i..n {
                for m in i-1..j {
                    if dp[i-1][m] != i32::MAX {
                        dp[i][j] = dp[i][j].min(dp[i-1][m] + cost[m+1][j]);
                    }
                }
            }
        }
        dp[k-1][n-1]
    }
}
```