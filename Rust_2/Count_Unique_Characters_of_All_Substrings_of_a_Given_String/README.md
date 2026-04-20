# Count Unique Characters of All Substrings of a Given String

**Difficulty:** Hard
**Tags:** Hash Table, String, Dynamic Programming

---

## Problem

<p>Let&#39;s define a function <code>countUniqueChars(s)</code> that returns the number of unique characters in&nbsp;<code>s</code>.</p>

<ul>
	<li>For example, calling <code>countUniqueChars(s)</code> if <code>s = &quot;LEETCODE&quot;</code> then <code>&quot;L&quot;</code>, <code>&quot;T&quot;</code>, <code>&quot;C&quot;</code>, <code>&quot;O&quot;</code>, <code>&quot;D&quot;</code> are the unique characters since they appear only once in <code>s</code>, therefore <code>countUniqueChars(s) = 5</code>.</li>
</ul>

<p>Given a string <code>s</code>, return the sum of <code>countUniqueChars(t)</code> where <code>t</code> is a substring of <code>s</code>. The test cases are generated such that the answer fits in a 32-bit integer.</p>

<p>Notice that some substrings can be repeated so in this case you have to count the repeated ones too.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ABC&quot;
<strong>Output:</strong> 10
<strong>Explanation: </strong>All possible substrings are: &quot;A&quot;,&quot;B&quot;,&quot;C&quot;,&quot;AB&quot;,&quot;BC&quot; and &quot;ABC&quot;.
Every substring is composed with only unique letters.
Sum of lengths of all substring is 1 + 1 + 1 + 2 + 2 + 3 = 10
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ABA&quot;
<strong>Output:</strong> 8
<strong>Explanation: </strong>The same as example 1, except <code>countUniqueChars</code>(&quot;ABA&quot;) = 1.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;LEETCODE&quot;
<strong>Output:</strong> 92
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of uppercase English letters only.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut last = [-1i64; 26];
        let mut prev = [-1i64; 26];
        let mut res = 0i64;

        for i in 0..n {
            let c = (s[i] - b'A') as usize;
            res += (i as i64 - last[c]) * (n as i64 - i as i64);
            if last[c] >= 0 {
                res -= (last[c] - prev[c]) * (n as i64 - i as i64);
            }
            prev[c] = last[c];
            last[c] = i as i64;
        }

        res as i32
    }
}
```