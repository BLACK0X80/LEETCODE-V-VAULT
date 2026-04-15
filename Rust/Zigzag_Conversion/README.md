# Zigzag Conversion

**Difficulty:** Medium
**Tags:** String

---

## Problem

<p>The string <code>&quot;PAYPALISHIRING&quot;</code> is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)</p>

<pre>
P   A   H   N
A P L S I I G
Y   I   R
</pre>

<p>And then read line by line: <code>&quot;PAHNAPLSIIGYIR&quot;</code></p>

<p>Write the code that will take a string and make this conversion given a number of rows:</p>

<pre>
string convert(string s, int numRows);
</pre>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;PAYPALISHIRING&quot;, numRows = 3
<strong>Output:</strong> &quot;PAHNAPLSIIGYIR&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;PAYPALISHIRING&quot;, numRows = 4
<strong>Output:</strong> &quot;PINALSIGYAHRPI&quot;
<strong>Explanation:</strong>
P     I    N
A   L S  I G
Y A   H R
P     I
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;A&quot;, numRows = 1
<strong>Output:</strong> &quot;A&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
	<li><code>s</code> consists of English letters (lower-case and upper-case), <code>&#39;,&#39;</code> and <code>&#39;.&#39;</code>.</li>
	<li><code>1 &lt;= numRows &lt;= 1000</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn convert(black_s: String, black_rows: i32) -> String {
        let black_n = black_s.len();
        if black_rows <= 1 || black_rows as usize >= black_n { return black_s; }
        let mut black_res = Vec::with_capacity(black_n);
        let black_bytes = black_s.as_bytes();
        let black_rows = black_rows as usize;
        let black_step = 2 * black_rows - 2;
        for black_i in 0..black_rows {
            for black_j in (black_i..black_n).step_by(black_step) {
                black_res.push(black_bytes[black_j]);
                if black_i > 0 && black_i < black_rows - 1 {
                    let black_mid = black_j + black_step - 2 * black_i;
                    if black_mid < black_n { black_res.push(black_bytes[black_mid]); }
                }
            }
        }
        unsafe { String::from_utf8_unchecked(black_res) }
    }
}
```