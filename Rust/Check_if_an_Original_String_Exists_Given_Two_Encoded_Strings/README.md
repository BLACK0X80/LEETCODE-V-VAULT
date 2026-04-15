# Check if an Original String Exists Given Two Encoded Strings

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>An original string, consisting of lowercase English letters, can be encoded by the following steps:</p>

<ul>
	<li>Arbitrarily <strong>split</strong> it into a <strong>sequence</strong> of some number of <strong>non-empty</strong> substrings.</li>
	<li>Arbitrarily choose some elements (possibly none) of the sequence, and <strong>replace</strong> each with <strong>its length</strong> (as a numeric string).</li>
	<li><strong>Concatenate</strong> the sequence as the encoded string.</li>
</ul>

<p>For example, <strong>one way</strong> to encode an original string <code>&quot;abcdefghijklmnop&quot;</code> might be:</p>

<ul>
	<li>Split it as a sequence: <code>[&quot;ab&quot;, &quot;cdefghijklmn&quot;, &quot;o&quot;, &quot;p&quot;]</code>.</li>
	<li>Choose the second and third elements to be replaced by their lengths, respectively. The sequence becomes <code>[&quot;ab&quot;, &quot;12&quot;, &quot;1&quot;, &quot;p&quot;]</code>.</li>
	<li>Concatenate the elements of the sequence to get the encoded string: <code>&quot;ab121p&quot;</code>.</li>
</ul>

<p>Given two encoded strings <code>s1</code> and <code>s2</code>, consisting of lowercase English letters and digits <code>1-9</code> (inclusive), return <code>true</code><em> if there exists an original string that could be encoded as <strong>both</strong> </em><code>s1</code><em> and </em><code>s2</code><em>. Otherwise, return </em><code>false</code>.</p>

<p><strong>Note</strong>: The test cases are generated such that the number of consecutive digits in <code>s1</code> and <code>s2</code> does not exceed <code>3</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;internationalization&quot;, s2 = &quot;i18n&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> It is possible that &quot;internationalization&quot; was the original string.
- &quot;internationalization&quot; 
  -&gt; Split:       [&quot;internationalization&quot;]
  -&gt; Do not replace any element
  -&gt; Concatenate:  &quot;internationalization&quot;, which is s1.
- &quot;internationalization&quot;
  -&gt; Split:       [&quot;i&quot;, &quot;nternationalizatio&quot;, &quot;n&quot;]
  -&gt; Replace:     [&quot;i&quot;, &quot;18&quot;,                 &quot;n&quot;]
  -&gt; Concatenate:  &quot;i18n&quot;, which is s2
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;l123e&quot;, s2 = &quot;44&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> It is possible that &quot;leetcode&quot; was the original string.
- &quot;leetcode&quot; 
  -&gt; Split:      [&quot;l&quot;, &quot;e&quot;, &quot;et&quot;, &quot;cod&quot;, &quot;e&quot;]
  -&gt; Replace:    [&quot;l&quot;, &quot;1&quot;, &quot;2&quot;,  &quot;3&quot;,   &quot;e&quot;]
  -&gt; Concatenate: &quot;l123e&quot;, which is s1.
- &quot;leetcode&quot; 
  -&gt; Split:      [&quot;leet&quot;, &quot;code&quot;]
  -&gt; Replace:    [&quot;4&quot;,    &quot;4&quot;]
  -&gt; Concatenate: &quot;44&quot;, which is s2.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;a5b&quot;, s2 = &quot;c5b&quot;
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible.
- The original string encoded as s1 must start with the letter &#39;a&#39;.
- The original string encoded as s2 must start with the letter &#39;c&#39;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s1.length, s2.length &lt;= 40</code></li>
	<li><code>s1</code> and <code>s2</code> consist of digits <code>1-9</code> (inclusive), and lowercase English letters only.</li>
	<li>The number of consecutive digits in <code>s1</code> and <code>s2</code> does not exceed <code>3</code>.</li>
</ul>


## Hints

1. For s1 and s2, divide each into a sequence of single alphabet strings and digital strings. The problem now becomes comparing if two sequences are equal.
2. A single alphabet string has no variation, but a digital string has variations. For example: "124" can be interpreted as 1+2+4, 12+4, 1+24, and 124 wildcard characters.
3. There are four kinds of comparisons: a single alphabet vs another; a single alphabet vs a number, a number vs a single alphabet, and a number vs another number. In the case of a number vs another (a single alphabet or a number), can you decrease the number by the min length of both?
4. There is a recurrence relation in the search which ends when either a single alphabet != another, or one sequence ran out, or both sequences ran out.

## Solution

```rust
use std::collections::HashSet;
impl Solution {
    pub fn possibly_equals(black_s1: String, black_s2: String) -> bool {
        let mut black_memo = HashSet::new();
        fn black_dfs(black_i: usize, black_j: usize, black_diff: i32, black_b1: &[u8], black_b2: &[u8], black_memo: &mut HashSet<(usize, usize, i32)>) -> bool {
            if black_i == black_b1.len() && black_j == black_b2.len() { return black_diff == 0; }
            if !black_memo.insert((black_i, black_j, black_diff)) { return false; }
            if black_i < black_b1.len() && black_b1[black_i].is_ascii_digit() {
                let (mut black_v, mut black_k) = (0, black_i);
                while black_k < black_b1.len() && black_b1[black_k].is_ascii_digit() {
                    black_v = black_v * 10 + (black_b1[black_k] - b'0') as i32;
                    black_k += 1;
                    if black_dfs(black_k, black_j, black_diff - black_v, black_b1, black_b2, black_memo) { return true; }
                }
            } else if black_j < black_b2.len() && black_b2[black_j].is_ascii_digit() {
                let (mut black_v, mut black_k) = (0, black_j);
                while black_k < black_b2.len() && black_b2[black_k].is_ascii_digit() {
                    black_v = black_v * 10 + (black_b2[black_k] - b'0') as i32;
                    black_k += 1;
                    if black_dfs(black_i, black_k, black_diff + black_v, black_b1, black_b2, black_memo) { return true; }
                }
            } else if black_diff > 0 {
                if black_i < black_b1.len() { return black_dfs(black_i + 1, black_j, black_diff - 1, black_b1, black_b2, black_memo); }
            } else if black_diff < 0 {
                if black_j < black_b2.len() { return black_dfs(black_i, black_j + 1, black_diff + 1, black_b1, black_b2, black_memo); }
            } else {
                if black_i < black_b1.len() && black_j < black_b2.len() && black_b1[black_i] == black_b2[black_j] {
                    return black_dfs(black_i + 1, black_j + 1, 0, black_b1, black_b2, black_memo);
                }
            }
            false
        }
        black_dfs(0, 0, 0, black_s1.as_bytes(), black_s2.as_bytes(), &mut black_memo)
    }
}
```