# Maximize Palindrome Length From Subsequences

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given two strings, <code>word1</code> and <code>word2</code>. You want to construct a string in the following manner:</p>

<ul>
	<li>Choose some <strong>non-empty</strong> subsequence <code>subsequence1</code> from <code>word1</code>.</li>
	<li>Choose some <strong>non-empty</strong> subsequence <code>subsequence2</code> from <code>word2</code>.</li>
	<li>Concatenate the subsequences: <code>subsequence1 + subsequence2</code>, to make the string.</li>
</ul>

<p>Return <em>the <strong>length</strong> of the longest <strong>palindrome</strong> that can be constructed in the described manner. </em>If no palindromes can be constructed, return <code>0</code>.</p>

<p>A <strong>subsequence</strong> of a string <code>s</code> is a string that can be made by deleting some (possibly none) characters from <code>s</code> without changing the order of the remaining characters.</p>

<p>A <strong>palindrome</strong> is a string that reads the same forward&nbsp;as well as backward.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> word1 = &quot;cacb&quot;, word2 = &quot;cbba&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> Choose &quot;ab&quot; from word1 and &quot;cba&quot; from word2 to make &quot;abcba&quot;, which is a palindrome.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> word1 = &quot;ab&quot;, word2 = &quot;ab&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> Choose &quot;ab&quot; from word1 and &quot;a&quot; from word2 to make &quot;aba&quot;, which is a palindrome.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> word1 = &quot;aa&quot;, word2 = &quot;bb&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong> You cannot construct a palindrome from the described method, so return 0.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word1.length, word2.length &lt;= 1000</code></li>
	<li><code>word1</code> and <code>word2</code> consist of lowercase English letters.</li>
</ul>


## Hints

1. Let's ignore the non-empty subsequence constraint. We can concatenate the two strings and find the largest palindromic subsequence with dynamic programming.
2. Iterate through every pair of characters word1[i] and word2[j], and see if some palindrome begins with word1[i] and ends with word2[j]. This ensures that the subsequences are non-empty.

## Solution

```rust
impl Solution {
    pub fn longest_palindrome(black_word1: String, black_word2: String) -> i32 {
        let black_combined = format!("{}{}", black_word1, black_word2);
        let black_bytes = black_combined.as_bytes();
        let black_n = black_bytes.len();
        let black_n1 = black_word1.len();
        let mut black_dp = vec![vec![0; black_n]; black_n];
        let mut black_res = 0;

        for black_i in (0..black_n).rev() {
            black_dp[black_i][black_i] = 1;
            for black_j in black_i + 1..black_n {
                if black_bytes[black_i] == black_bytes[black_j] {
                    black_dp[black_i][black_j] = black_dp[black_i + 1][black_j - 1] + 2;
                    if black_i < black_n1 && black_j >= black_n1 {
                        black_res = black_res.max(black_dp[black_i][black_j]);
                    }
                } else {
                    black_dp[black_i][black_j] = black_dp[black_i + 1][black_j].max(black_dp[black_i][black_j - 1]);
                }
            }
        }
        black_res
    }
}
```