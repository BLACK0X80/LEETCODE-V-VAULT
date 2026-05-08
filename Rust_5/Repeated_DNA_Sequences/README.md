# Repeated DNA Sequences

**Difficulty:** Medium
**Tags:** Hash Table, String, Bit Manipulation, Sliding Window, Rolling Hash, Hash Function

---

## Problem

<p>The <strong>DNA sequence</strong> is composed of a series of nucleotides abbreviated as <code>&#39;A&#39;</code>, <code>&#39;C&#39;</code>, <code>&#39;G&#39;</code>, and <code>&#39;T&#39;</code>.</p>

<ul>
	<li>For example, <code>&quot;ACGAATTCCG&quot;</code> is a <strong>DNA sequence</strong>.</li>
</ul>

<p>When studying <strong>DNA</strong>, it is useful to identify repeated sequences within the DNA.</p>

<p>Given a string <code>s</code> that represents a <strong>DNA sequence</strong>, return all the <strong><code>10</code>-letter-long</strong> sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
<strong>Output:</strong> ["AAAAACCCCC","CCCCCAAAAA"]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> s = "AAAAAAAAAAAAA"
<strong>Output:</strong> ["AAAAAAAAAA"]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s[i]</code> is either <code>&#39;A&#39;</code>, <code>&#39;C&#39;</code>, <code>&#39;G&#39;</code>, or <code>&#39;T&#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn find_repeated_dna_sequences(black_s: String) -> Vec<String> { if black_s.len() < 10 { return vec![]; } let (mut black_m, mut black_seen, mut black_res, black_b) = ([0u8; 1 << 20], [false; 1 << 20], vec![], black_s.as_bytes()); let black_map = |black_c| match black_c { b'A' => 0, b'C' => 1, b'G' => 2, _ => 3 }; let mut black_h = 0; for i in 0..9 { black_h = (black_h << 2) | black_map(black_b[i]); } for i in 9..black_b.len() { black_h = ((black_h << 2) & 0xFFFFF) | black_map(black_b[i]); if black_m[black_h] == 1 { black_res.push(black_s[i-9..i+1].to_string()); } if black_m[black_h] < 2 { black_m[black_h] += 1; } } black_res } }
```