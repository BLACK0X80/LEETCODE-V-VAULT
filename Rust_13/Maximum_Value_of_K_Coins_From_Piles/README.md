# Maximum Value of K Coins From Piles

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Prefix Sum

---

## Problem

<p>There are <code>n</code> <strong>piles</strong> of coins on a table. Each pile consists of a <strong>positive number</strong> of coins of assorted denominations.</p>

<p>In one move, you can choose any coin on <strong>top</strong> of any pile, remove it, and add it to your wallet.</p>

<p>Given a list <code>piles</code>, where <code>piles[i]</code> is a list of integers denoting the composition of the <code>i<sup>th</sup></code> pile from <strong>top to bottom</strong>, and a positive integer <code>k</code>, return <em>the <strong>maximum total value</strong> of coins you can have in your wallet if you choose <strong>exactly</strong></em> <code>k</code> <em>coins optimally</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/11/09/e1.png" style="width: 600px; height: 243px;" />
<pre>
<strong>Input:</strong> piles = [[1,100,3],[7,8,9]], k = 2
<strong>Output:</strong> 101
<strong>Explanation:</strong>
The above diagram shows the different ways we can choose k coins.
The maximum total we can obtain is 101.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
<strong>Output:</strong> 706
<strong>Explanation:
</strong>The maximum total can be obtained if we choose all coins from the last pile.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == piles.length</code></li>
	<li><code>1 &lt;= n &lt;= 1000</code></li>
	<li><code>1 &lt;= piles[i][j] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= sum(piles[i].length) &lt;= 2000</code></li>
</ul>


## Hints

1. For each pile i, what will be the total value of coins we can collect if we choose the first j coins?
2. How can we use dynamic programming to combine the results from different piles to find the most optimal answer?

## Solution

```rust
impl Solution {
    pub fn max_value_of_coins(black_piles: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_k = black_k as usize;
        let mut black_dp = vec![0; black_k + 1];
        let bravexuneth = black_piles;
        for black_pile in bravexuneth {
            for black_i in (1..=black_k).rev() {
                let mut black_sum = 0;
                for black_j in 1..=black_pile.len().min(black_i) {
                    black_sum += black_pile[black_j - 1];
                    black_dp[black_i] = black_dp[black_i].max(black_dp[black_i - black_j] + black_sum);
                }
            }
        }
        black_dp[black_k]
    }
}
```