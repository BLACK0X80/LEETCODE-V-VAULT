# Maximum Product of Word Lengths

**Difficulty:** Medium
**Tags:** Array, String, Bit Manipulation

---

## Problem

<p>Given a string array <code>words</code>, return <em>the maximum value of</em> <code>length(word[i]) * length(word[j])</code> <em>where the two words do not share common letters</em>. If no such two words exist, return <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;abcw&quot;,&quot;baz&quot;,&quot;foo&quot;,&quot;bar&quot;,&quot;xtfn&quot;,&quot;abcdef&quot;]
<strong>Output:</strong> 16
<strong>Explanation:</strong> The two words can be &quot;abcw&quot;, &quot;xtfn&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;a&quot;,&quot;ab&quot;,&quot;abc&quot;,&quot;d&quot;,&quot;cd&quot;,&quot;bcd&quot;,&quot;abcd&quot;]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The two words can be &quot;ab&quot;, &quot;cd&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;a&quot;,&quot;aa&quot;,&quot;aaa&quot;,&quot;aaaa&quot;]
<strong>Output:</strong> 0
<strong>Explanation:</strong> No such pair of words.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= words.length &lt;= 1000</code></li>
	<li><code>1 &lt;= words[i].length &lt;= 1000</code></li>
	<li><code>words[i]</code> consists only of lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution { pub fn max_product(words: Vec<String>) -> i32 { let (mut black_m, mut black_res) = (words.iter().map(|w| (w.bytes().fold(0, |a, b| a | (1 << (b - b'a'))), w.len() as i32)).collect::<Vec<_>>(), 0); for black_i in 0..black_m.len() { for black_j in black_i + 1..black_m.len() { if black_m[black_i].0 & black_m[black_j].0 == 0 { black_res = black_res.max(black_m[black_i].1 * black_m[black_j].1); } } } black_res } }
```