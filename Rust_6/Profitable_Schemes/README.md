# Profitable Schemes

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>There is a group of <code>n</code> members, and a list of various crimes they could commit. The <code>i<sup>th</sup></code> crime generates a <code>profit[i]</code> and requires <code>group[i]</code> members to participate in it. If a member participates in one crime, that member can&#39;t participate in another crime.</p>

<p>Let&#39;s call a <strong>profitable scheme</strong> any subset of these crimes that generates at least <code>minProfit</code> profit, and the total number of members participating in that subset of crimes is at most <code>n</code>.</p>

<p>Return the number of schemes that can be chosen. Since the answer may be very large, <strong>return it modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 5, minProfit = 3, group = [2,2], profit = [2,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> To make a profit of at least 3, the group could either commit crimes 0 and 1, or just crime 1.
In total, there are 2 schemes.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
<strong>Output:</strong> 7
<strong>Explanation:</strong> To make a profit of at least 5, the group could commit any crimes, as long as they commit one.
There are 7 possible schemes: (0), (1), (2), (0,1), (0,2), (1,2), and (0,1,2).</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 100</code></li>
	<li><code>0 &lt;= minProfit &lt;= 100</code></li>
	<li><code>1 &lt;= group.length &lt;= 100</code></li>
	<li><code>1 &lt;= group[i] &lt;= 100</code></li>
	<li><code>profit.length == group.length</code></li>
	<li><code>0 &lt;= profit[i] &lt;= 100</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let (n, mp) = (n as usize, min_profit as usize);
        let mut dp = vec![vec![0u64; mp + 1]; n + 1];
        dp[0][0] = 1;

        for i in 0..group.len() {
            let (g, p) = (group[i] as usize, profit[i] as usize);
            for members in (0..=n).rev() {
                if members + g > n { continue; }
                for prof in (0..=mp).rev() {
                    let np = (prof + p).min(mp);
                    dp[members + g][np] = (dp[members + g][np] + dp[members][prof]) % MOD;
                }
            }
        }

        let mut ans = 0u64;
        for m in 0..=n { ans = (ans + dp[m][mp]) % MOD; }
        ans as i32
    }
}
```