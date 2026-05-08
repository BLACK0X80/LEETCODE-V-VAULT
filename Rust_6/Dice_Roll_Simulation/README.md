# Dice Roll Simulation

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>A die simulator generates a random number from <code>1</code> to <code>6</code> for each roll. You introduced a constraint to the generator such that it cannot roll the number <code>i</code> more than <code>rollMax[i]</code> (<strong>1-indexed</strong>) consecutive times.</p>

<p>Given an array of integers <code>rollMax</code> and an integer <code>n</code>, return <em>the number of distinct sequences that can be obtained with exact </em><code>n</code><em> rolls</em>. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>Two sequences are considered different if at least one element differs from each other.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2, rollMax = [1,1,2,2,2,3]
<strong>Output:</strong> 34
<strong>Explanation:</strong> There will be 2 rolls of die, if there are no constraints on the die, there are 6 * 6 = 36 possible combinations. In this case, looking at rollMax array, the numbers 1 and 2 appear at most once consecutively, therefore sequences (1,1) and (2,2) cannot occur, so the final answer is 36-2 = 34.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2, rollMax = [1,1,1,1,1,1]
<strong>Output:</strong> 30
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 3, rollMax = [1,1,1,2,2,3]
<strong>Output:</strong> 181
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 5000</code></li>
	<li><code>rollMax.length == 6</code></li>
	<li><code>1 &lt;= rollMax[i] &lt;= 15</code></li>
</ul>


## Hints

1. Think on Dynamic Programming.
2. DP(pos, last) which means we are at the position pos having as last the last character seen.

## Solution

```rust
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        const M: u64 = 1_000_000_007;
        let n = n as usize;
        let mut dp = vec![[0u64; 6]; n + 1];
        for j in 0..6 { dp[1][j] = 1; }

        for i in 2..=n {
            for j in 0..6 {
                for k in 1..=roll_max[j] as usize {
                    if i < k { break; }
                    if k == i {
                        dp[i][j] = (dp[i][j] + 1) % M;
                    } else {
                        let sum: u64 = (0..6).filter(|&f| f != j)
                            .map(|f| dp[i-k][f]).sum::<u64>() % M;
                        dp[i][j] = (dp[i][j] + sum) % M;
                    }
                }
            }
        }

        (dp[n].iter().sum::<u64>() % M) as i32
    }
}
```