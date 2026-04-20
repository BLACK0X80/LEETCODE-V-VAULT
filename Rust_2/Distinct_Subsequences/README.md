# Distinct Subsequences

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given two strings s and t, return <i>the number of distinct</i> <b><i>subsequences</i></b><i> of </i>s<i> which equals </i>t.</p>

<p>The test cases are generated so that the answer fits on a 32-bit signed integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;rabbbit&quot;, t = &quot;rabbit&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong>
As shown below, there are 3 ways you can generate &quot;rabbit&quot; from s.
<code><strong><u>rabb</u></strong>b<strong><u>it</u></strong></code>
<code><strong><u>ra</u></strong>b<strong><u>bbit</u></strong></code>
<code><strong><u>rab</u></strong>b<strong><u>bit</u></strong></code>
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;babgbag&quot;, t = &quot;bag&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong>
As shown below, there are 5 ways you can generate &quot;bag&quot; from s.
<code><strong><u>ba</u></strong>b<u><strong>g</strong></u>bag</code>
<code><strong><u>ba</u></strong>bgba<strong><u>g</u></strong></code>
<code><u><strong>b</strong></u>abgb<strong><u>ag</u></strong></code>
<code>ba<u><strong>b</strong></u>gb<u><strong>ag</strong></u></code>
<code>babg<strong><u>bag</u></strong></code></pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length, t.length &lt;= 1000</code></li>
	<li><code>s</code> and <code>t</code> consist of English letters.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let (m, n) = (s.len(), t.len());
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        for i in 0..m {
            for j in (0..n).rev() {
                if s[i] == t[j] {
                    dp[j + 1] += dp[j];
                }
            }
        }

        dp[n] as i32
    }
}
```