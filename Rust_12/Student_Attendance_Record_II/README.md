# Student Attendance Record II

**Difficulty:** Hard
**Tags:** Dynamic Programming

---

## Problem

<p>An attendance record for a student can be represented as a string where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:</p>

<ul>
	<li><code>&#39;A&#39;</code>: Absent.</li>
	<li><code>&#39;L&#39;</code>: Late.</li>
	<li><code>&#39;P&#39;</code>: Present.</li>
</ul>

<p>Any student is eligible for an attendance award if they meet <strong>both</strong> of the following criteria:</p>

<ul>
	<li>The student was absent (<code>&#39;A&#39;</code>) for <strong>strictly</strong> fewer than 2 days <strong>total</strong>.</li>
	<li>The student was <strong>never</strong> late (<code>&#39;L&#39;</code>) for 3 or more <strong>consecutive</strong> days.</li>
</ul>

<p>Given an integer <code>n</code>, return <em>the <strong>number</strong> of possible attendance records of length</em> <code>n</code><em> that make a student eligible for an attendance award. The answer may be very large, so return it <strong>modulo</strong> </em><code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong> There are 8 records with length 2 that are eligible for an award:
&quot;PP&quot;, &quot;AP&quot;, &quot;PA&quot;, &quot;LP&quot;, &quot;PL&quot;, &quot;AL&quot;, &quot;LA&quot;, &quot;LL&quot;
Only &quot;AA&quot; is not eligible because there are 2 absences (there need to be fewer than 2).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 3
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 10101
<strong>Output:</strong> 183236316
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut dp = [[0u64; 3]; 2];
        dp[0][0] = 1;

        for _ in 0..n {
            let p = dp;
            dp = [[0; 3]; 2];
            for a in 0..2usize {
                for l in 0..3usize {
                    if p[a][l] == 0 { continue; }
                    let v = p[a][l];
                    dp[a][0] = (dp[a][0] + v) % MOD;
                    if a == 0 { dp[1][0] = (dp[1][0] + v) % MOD; }
                    if l < 2 { dp[a][l+1] = (dp[a][l+1] + v) % MOD; }
                }
            }
        }

        let mut ans = 0u64;
        for a in 0..2 { for l in 0..3 { ans = (ans + dp[a][l]) % MOD; } }
        ans as i32
    }
}
```