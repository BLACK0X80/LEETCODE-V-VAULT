# Maximum Deletions on a String

**Difficulty:** Hard
**Tags:** String, Dynamic Programming, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>You are given a string <code>s</code> consisting of only lowercase English letters. In one operation, you can:</p>

<ul>
	<li>Delete <strong>the entire string</strong> <code>s</code>, or</li>
	<li>Delete the <strong>first</strong> <code>i</code> letters of <code>s</code> if the first <code>i</code> letters of <code>s</code> are <strong>equal</strong> to the following <code>i</code> letters in <code>s</code>, for any <code>i</code> in the range <code>1 &lt;= i &lt;= s.length / 2</code>.</li>
</ul>

<p>For example, if <code>s = &quot;ababc&quot;</code>, then in one operation, you could delete the first two letters of <code>s</code> to get <code>&quot;abc&quot;</code>, since the first two letters of <code>s</code> and the following two letters of <code>s</code> are both equal to <code>&quot;ab&quot;</code>.</p>

<p>Return <em>the <strong>maximum</strong> number of operations needed to delete all of </em><code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcabcdabc&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- Delete the first 3 letters (&quot;abc&quot;) since the next 3 letters are equal. Now, s = &quot;abcdabc&quot;.
- Delete all the letters.
We used 2 operations so return 2. It can be proven that 2 is the maximum number of operations needed.
Note that in the second operation we cannot delete &quot;abc&quot; again because the next occurrence of &quot;abc&quot; does not happen in the next 3 letters.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aaabaab&quot;
<strong>Output:</strong> 4
<strong>Explanation:</strong>
- Delete the first letter (&quot;a&quot;) since the next letter is equal. Now, s = &quot;aabaab&quot;.
- Delete the first 3 letters (&quot;aab&quot;) since the next 3 letters are equal. Now, s = &quot;aab&quot;.
- Delete the first letter (&quot;a&quot;) since the next letter is equal. Now, s = &quot;ab&quot;.
- Delete all the letters.
We used 4 operations so return 4. It can be proven that 4 is the maximum number of operations needed.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aaaaa&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> In each operation, we can delete the first letter of s.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 4000</code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. We can use dynamic programming to find the answer. Create a 0-indexed dp array where dp[i] represents the maximum number of moves needed to remove the first i + 1 letters from s.
2. What should we do if there is an i where it is impossible to remove the first i + 1 letters?
3. Use a sentinel value such as -1 to show that it is impossible.
4. How can we quickly determine if two substrings of s are equal? We can use hashing.

## Solution

```rust
impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut lcp = vec![vec![0usize; n+1]; n+1];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if s[i] == s[j] { lcp[i][j] = lcp[i+1][j+1] + 1; }
            }
        }
        let mut dp = vec![1usize; n];
        for i in (0..n-1).rev() {
            for l in 1..=(n-i)/2 {
                if lcp[i][i+l] >= l {
                    dp[i] = dp[i].max(1 + dp[i+l]);
                }
            }
        }
        dp[0] as i32
    }
}
```