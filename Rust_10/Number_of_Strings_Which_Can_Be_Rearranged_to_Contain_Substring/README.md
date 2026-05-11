# Number of Strings Which Can Be Rearranged to Contain Substring

**Difficulty:** Medium
**Tags:** Math, Dynamic Programming, Combinatorics

---

## Problem

<p>You are given an integer <code>n</code>.</p>

<p>A string <code>s</code> is called <strong>good </strong>if it contains only lowercase English characters <strong>and</strong> it is possible to rearrange the characters of <code>s</code> such that the new string contains <code>&quot;leet&quot;</code> as a <strong>substring</strong>.</p>

<p>For example:</p>

<ul>
	<li>The string <code>&quot;lteer&quot;</code> is good because we can rearrange it to form <code>&quot;leetr&quot;</code> .</li>
	<li><code>&quot;letl&quot;</code> is not good because we cannot rearrange it to contain <code>&quot;leet&quot;</code> as a substring.</li>
</ul>

<p>Return <em>the <strong>total</strong> number of good strings of length </em><code>n</code>.</p>

<p>Since the answer may be large, return it <strong>modulo </strong><code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters within a string.</p>

<div class="notranslate" style="all: initial;">&nbsp;</div>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 12
<strong>Explanation:</strong> The 12 strings which can be rearranged to have &quot;leet&quot; as a substring are: &quot;eelt&quot;, &quot;eetl&quot;, &quot;elet&quot;, &quot;elte&quot;, &quot;etel&quot;, &quot;etle&quot;, &quot;leet&quot;, &quot;lete&quot;, &quot;ltee&quot;, &quot;teel&quot;, &quot;tele&quot;, and &quot;tlee&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 83943898
<strong>Explanation:</strong> The number of strings with length 10 which can be rearranged to have &quot;leet&quot; as a substring is 526083947580. Hence the answer is 526083947580 % (10<sup>9</sup> + 7) = 83943898.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. A good string must contain at least one <code>l</code>, one <code>t</code>, and two <code>e</code>.
2. Divide the problem into subproblems and use Dynamic Programming.

## Solution

```rust
impl Solution { pub fn string_count(n: i32) -> i32 { let mut black_dp = vec![vec![vec![vec![-1i64; 2]; 3]; 2]; (n + 1) as usize]; let black_m = 1_000_000_007i64; fn solve(i: usize, l: usize, e: usize, t: usize, n: usize, dp: &mut Vec<Vec<Vec<Vec<i64>>>>, m: i64) -> i64 { if i == n { return if l == 1 && e == 2 && t == 1 { 1 } else { 0 }; } if dp[i][l][e][t] != -1 { return dp[i][l][e][t]; } let mut res = (solve(i + 1, l, e, t, n, dp, m) * 23) % m; res = (res + solve(i + 1, 1.max(l), e, t, n, dp, m)) % m; res = (res + solve(i + 1, l, 2.min(e + 1), t, n, dp, m)) % m; res = (res + solve(i + 1, l, e, 1.max(t), n, dp, m)) % m; dp[i][l][e][t] = res; res } (solve(0, 0, 0, 0, n as usize, &mut black_dp, black_m)) as i32 } }
```