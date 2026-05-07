# Lexicographically Smallest Beautiful String

**Difficulty:** Hard
**Tags:** String, Greedy

---

## Problem

<p>A string is <strong>beautiful</strong> if:</p>

<ul>
	<li>It consists of the first <code>k</code> letters of the English lowercase alphabet.</li>
	<li>It does not contain any substring of length <code>2</code> or more which is a palindrome.</li>
</ul>

<p>You are given a beautiful string <code>s</code> of length <code>n</code> and a positive integer <code>k</code>.</p>

<p>Return <em>the lexicographically smallest string of length </em><code>n</code><em>, which is larger than </em><code>s</code><em> and is <strong>beautiful</strong></em>. If there is no such string, return an empty string.</p>

<p>A string <code>a</code> is lexicographically larger than a string <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, <code>a</code> has a character strictly larger than the corresponding character in <code>b</code>.</p>

<ul>
	<li>For example, <code>&quot;abcd&quot;</code> is lexicographically larger than <code>&quot;abcc&quot;</code> because the first position they differ is at the fourth character, and <code>d</code> is greater than <code>c</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcz&quot;, k = 26
<strong>Output:</strong> &quot;abda&quot;
<strong>Explanation:</strong> The string &quot;abda&quot; is beautiful and lexicographically larger than the string &quot;abcz&quot;.
It can be proven that there is no string that is lexicographically larger than the string &quot;abcz&quot;, beautiful, and lexicographically smaller than the string &quot;abda&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;dc&quot;, k = 4
<strong>Output:</strong> &quot;&quot;
<strong>Explanation:</strong> It can be proven that there is no string that is lexicographically larger than the string &quot;dc&quot; and is beautiful.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>4 &lt;= k &lt;= 26</code></li>
	<li><code>s</code> is a beautiful string.</li>
</ul>


## Hints

1. If the string does not contain any palindromic substrings of lengths 2 and 3, then the string does not contain any palindromic substrings at all.
2. Iterate from right to left and if it is possible to increase character at index i without creating any palindromic substrings of lengths 2 and 3, then increase it.
3. After increasing the character at index i, set every character after index i equal to character a. With this, we will ensure that we have created a lexicographically larger string than s, which does not contain any palindromes before index i and is lexicographically the smallest.
4. Finally, we are just left with a case to fix palindromic substrings, which come after index i. This can be done with a similar method mentioned in the second hint.

## Solution

```rust
impl Solution {
    pub fn smallest_beautiful_string(black_s: String, black_k: i32) -> String {
        let mut black_bytes = black_s.into_bytes();
        let black_n = black_bytes.len();
        let black_limit = b'a' + black_k as u8;
        let mut black_idx = (black_n - 1) as isize;
        
        while black_idx >= 0 {
            black_bytes[black_idx as usize] += 1;
            while black_bytes[black_idx as usize] < black_limit && 
                  ((black_idx > 0 && black_bytes[black_idx as usize] == black_bytes[black_idx as usize - 1]) || 
                   (black_idx > 1 && black_bytes[black_idx as usize] == black_bytes[black_idx as usize - 2])) {
                black_bytes[black_idx as usize] += 1;
            }
            if black_bytes[black_idx as usize] < black_limit {
                for black_i in (black_idx as usize + 1)..black_n {
                    let mut black_c = b'a';
                    while (black_i > 0 && black_c == black_bytes[black_i - 1]) || 
                          (black_i > 1 && black_c == black_bytes[black_i - 2]) {
                        black_c += 1;
                    }
                    black_bytes[black_i] = black_c;
                }
                return String::from_utf8(black_bytes).unwrap();
            }
            black_idx -= 1;
        }
        "".to_string()
    }
}
```