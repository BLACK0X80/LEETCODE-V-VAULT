# Minimum Insertion Steps to Make a String Palindrome

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given a string <code>s</code>. In one step you can insert any character at any index of the string.</p>

<p>Return <em>the minimum number of steps</em> to make <code>s</code>&nbsp;palindrome.</p>

<p>A&nbsp;<b>Palindrome String</b>&nbsp;is one that reads the same backward as well as forward.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;zzazz&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong> The string &quot;zzazz&quot; is already palindrome we do not need any insertions.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;mbadm&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> String can be &quot;mbdadbm&quot; or &quot;mdbabdm&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;leetcode&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> Inserting 5 characters the string becomes &quot;leetcodocteel&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 500</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Is dynamic programming suitable for this problem ?
2. If we know the longest palindromic sub-sequence is x and the length of the string is n then, what is the answer to this problem? It is n - x as we need n - x insertions to make the remaining characters also palindrome.

## Solution

```rust
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let black_bytes = s.as_bytes();
        let black_n = black_bytes.len();
        let mut black_dp = vec![vec![0; black_n]; black_n];
        for black_len in 2..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len - 1;
                if black_bytes[black_i] == black_bytes[black_j] {
                    black_dp[black_i][black_j] = if black_len == 2 { 0 } else { black_dp[black_i + 1][black_j - 1] };
                } else {
                    black_dp[black_i][black_j] = 1 + black_dp[black_i + 1][black_j].min(black_dp[black_i][black_j - 1]);
                }
            }
        }
        black_dp[0][black_n - 1]
    }
}
```