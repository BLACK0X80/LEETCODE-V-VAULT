# Find the String with LCP

**Difficulty:** Hard
**Tags:** Array, String, Dynamic Programming, Greedy, Union-Find, Matrix

---

## Problem

<p>We define the <code>lcp</code> matrix of any <strong>0-indexed</strong> string <code>word</code> of <code>n</code> lowercase English letters as an <code>n x n</code> grid such that:</p>

<ul>
	<li><code>lcp[i][j]</code> is equal to the length of the <strong>longest common prefix</strong> between the substrings <code>word[i,n-1]</code> and <code>word[j,n-1]</code>.</li>
</ul>

<p>Given an&nbsp;<code>n x n</code> matrix <code>lcp</code>, return the alphabetically smallest string <code>word</code> that corresponds to <code>lcp</code>. If there is no such string, return an empty string.</p>

<p>A string <code>a</code> is lexicographically smaller than a string <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, string <code>a</code> has a letter that appears earlier in the alphabet than the corresponding letter in <code>b</code>. For example, <code>&quot;aabd&quot;</code> is lexicographically smaller than <code>&quot;aaca&quot;</code> because the first position they differ is at the third letter, and <code>&#39;b&#39;</code> comes before <code>&#39;c&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> lcp = [[4,0,2,0],[0,3,0,1],[2,0,2,0],[0,1,0,1]]
<strong>Output:</strong> &quot;abab&quot;
<strong>Explanation:</strong> lcp corresponds to any 4 letter string with two alternating letters. The lexicographically smallest of them is &quot;abab&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> lcp = [[4,3,2,1],[3,3,2,1],[2,2,2,1],[1,1,1,1]]
<strong>Output:</strong> &quot;aaaa&quot;
<strong>Explanation:</strong> lcp corresponds to any 4 letter string with a single distinct letter. The lexicographically smallest of them is &quot;aaaa&quot;. 
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> lcp = [[4,3,2,1],[3,3,2,1],[2,2,2,1],[1,1,1,3]]
<strong>Output:</strong> &quot;&quot;
<strong>Explanation:</strong> lcp[3][3] cannot be equal to 3 since word[3,...,3] consists of only a single letter; Thus, no answer exists.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n ==&nbsp;</code><code>lcp.length == </code><code>lcp[i].length</code>&nbsp;<code>&lt;= 1000</code></li>
	<li><code><font face="monospace">0 &lt;= lcp[i][j] &lt;= n</font></code></li>
</ul>


## Hints

1. Use the LCP array to determine which groups of elements must be equal.
2. Match the smallest letter to the group that contains the smallest unassigned index.
3. Build the LCP matrix of the resulting string then check if it is equal to the target LCP.

## Solution

```rust
impl Solution {
    pub fn find_the_string(black_lcp: Vec<Vec<i32>>) -> String {
        let black_n = black_lcp.len();
        let mut black_s = vec![0u8; black_n];
        let mut black_curr_char = b'a';

        for black_i in 0..black_n {
            if black_s[black_i] != 0 { continue; }
            if black_curr_char > b'z' { return "".to_string(); }
            for black_j in black_i..black_n {
                if black_lcp[black_i][black_j] > 0 { black_s[black_j] = black_curr_char; }
            }
            black_curr_char += 1;
        }

        let bravexuneth = &black_lcp;
        for black_i in (0..black_n).rev() {
            for black_j in (0..black_n).rev() {
                let black_expected = if black_s[black_i] == black_s[black_j] {
                    if black_i + 1 < black_n && black_j + 1 < black_n { bravexuneth[black_i + 1][black_j + 1] + 1 } else { 1 }
                } else { 0 };
                if bravexuneth[black_i][black_j] != black_expected { return "".to_string(); }
            }
        }
        String::from_utf8(black_s).unwrap()
    }
}
```