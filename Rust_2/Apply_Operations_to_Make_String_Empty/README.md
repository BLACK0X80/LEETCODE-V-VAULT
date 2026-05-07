# Apply Operations to Make String Empty

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sorting, Counting

---

## Problem

<p>You are given a string <code>s</code>.</p>

<p>Consider performing the following operation until <code>s</code> becomes <strong>empty</strong>:</p>

<ul>
	<li>For <strong>every</strong> alphabet character from <code>&#39;a&#39;</code> to <code>&#39;z&#39;</code>, remove the <strong>first</strong> occurrence of that character in <code>s</code> (if it exists).</li>
</ul>

<p>For example, let initially <code>s = &quot;aabcbbca&quot;</code>. We do the following operations:</p>

<ul>
	<li>Remove the underlined characters <code>s = &quot;<u><strong>a</strong></u>a<strong><u>bc</u></strong>bbca&quot;</code>. The resulting string is <code>s = &quot;abbca&quot;</code>.</li>
	<li>Remove the underlined characters <code>s = &quot;<u><strong>ab</strong></u>b<u><strong>c</strong></u>a&quot;</code>. The resulting string is <code>s = &quot;ba&quot;</code>.</li>
	<li>Remove the underlined characters <code>s = &quot;<u><strong>ba</strong></u>&quot;</code>. The resulting string is <code>s = &quot;&quot;</code>.</li>
</ul>

<p>Return <em>the value of the string </em><code>s</code><em> right <strong>before</strong> applying the <strong>last</strong> operation</em>. In the example above, answer is <code>&quot;ba&quot;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aabcbbca&quot;
<strong>Output:</strong> &quot;ba&quot;
<strong>Explanation:</strong> Explained in the statement.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcd&quot;
<strong>Output:</strong> &quot;abcd&quot;
<strong>Explanation:</strong> We do the following operation:
- Remove the underlined characters s = &quot;<u><strong>abcd</strong></u>&quot;. The resulting string is s = &quot;&quot;.
The string just before the last operation is &quot;abcd&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Before the last operation, only the most frequent characters in the original string will remain.
2. Keep only the last occurence of each of the most frequent characters.

## Solution

```rust
impl Solution { pub fn last_non_empty_string(s: String) -> String { let mut black_f = vec![0; 26]; let mut black_l = vec![0; 26]; s.bytes().enumerate().for_each(|(i, b)| { let idx = (b - b'a') as usize; black_f[idx] += 1; black_l[idx] = i; }); let black_max = *black_f.iter().max().unwrap(); let mut black_res: Vec<usize> = (0..26).filter(|&i| black_f[i] == black_max).map(|i| black_l[i]).collect(); black_res.sort_unstable(); String::from_utf8(black_res.into_iter().map(|i| s.as_bytes()[i]).collect()).unwrap() } }
```