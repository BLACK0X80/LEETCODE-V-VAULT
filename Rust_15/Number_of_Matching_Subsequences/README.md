# Number of Matching Subsequences

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Binary Search, Dynamic Programming, Trie, Sorting

---

## Problem

<p>Given a string <code>s</code> and an array of strings <code>words</code>, return <em>the number of</em> <code>words[i]</code> <em>that is a subsequence of</em> <code>s</code>.</p>

<p>A <strong>subsequence</strong> of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.</p>

<ul>
	<li>For example, <code>&quot;ace&quot;</code> is a subsequence of <code>&quot;abcde&quot;</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcde&quot;, words = [&quot;a&quot;,&quot;bb&quot;,&quot;acd&quot;,&quot;ace&quot;]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three strings in words that are a subsequence of s: &quot;a&quot;, &quot;acd&quot;, &quot;ace&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;dsahjpjauf&quot;, words = [&quot;ahjpjau&quot;,&quot;ja&quot;,&quot;ahbwzgqnuk&quot;,&quot;tnmlanowax&quot;]
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words.length &lt;= 5000</code></li>
	<li><code>1 &lt;= words[i].length &lt;= 50</code></li>
	<li><code>s</code> and <code>words[i]</code> consist of only lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution { pub fn num_matching_subseq(black_s: String, words: Vec<String>) -> i32 { let mut black_buckets: Vec<Vec<std::str::Chars>> = vec![vec![]; 26]; for w in &words { black_buckets[(w.chars().next().unwrap() as u8 - b'a') as usize].push(w.chars()); } let mut black_ans = 0; for c in black_s.chars() { let mut black_curr = std::mem::take(&mut black_buckets[(c as u8 - b'a') as usize]); for mut it in black_curr { it.next(); if let Some(next_c) = it.clone().next() { black_buckets[(next_c as u8 - b'a') as usize].push(it); } else { black_ans += 1; } } } black_ans } }
```