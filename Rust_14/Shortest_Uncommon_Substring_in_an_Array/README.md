# Shortest Uncommon Substring in an Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Trie

---

## Problem

<p>You are given an array <code>arr</code> of size <code>n</code> consisting of <strong>non-empty</strong> strings.</p>

<p>Find a string array <code>answer</code> of size <code>n</code> such that:</p>

<ul>
	<li><code>answer[i]</code> is the <strong>shortest</strong> <span data-keyword="substring">substring</span> of <code>arr[i]</code> that does <strong>not</strong> occur as a substring in any other string in <code>arr</code>. If multiple such substrings exist, <code>answer[i]</code> should be the <span data-keyword="lexicographically-smaller-string">lexicographically smallest</span>. And if no such substring exists, <code>answer[i]</code> should be an empty string.</li>
</ul>

<p>Return <em>the array </em><code>answer</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [&quot;cab&quot;,&quot;ad&quot;,&quot;bad&quot;,&quot;c&quot;]
<strong>Output:</strong> [&quot;ab&quot;,&quot;&quot;,&quot;ba&quot;,&quot;&quot;]
<strong>Explanation:</strong> We have the following:
- For the string &quot;cab&quot;, the shortest substring that does not occur in any other string is either &quot;ca&quot; or &quot;ab&quot;, we choose the lexicographically smaller substring, which is &quot;ab&quot;.
- For the string &quot;ad&quot;, there is no substring that does not occur in any other string.
- For the string &quot;bad&quot;, the shortest substring that does not occur in any other string is &quot;ba&quot;.
- For the string &quot;c&quot;, there is no substring that does not occur in any other string.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [&quot;abc&quot;,&quot;bcd&quot;,&quot;abcd&quot;]
<strong>Output:</strong> [&quot;&quot;,&quot;&quot;,&quot;abcd&quot;]
<strong>Explanation:</strong> We have the following:
- For the string &quot;abc&quot;, there is no substring that does not occur in any other string.
- For the string &quot;bcd&quot;, there is no substring that does not occur in any other string.
- For the string &quot;abcd&quot;, the shortest substring that does not occur in any other string is &quot;abcd&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == arr.length</code></li>
	<li><code>2 &lt;= n &lt;= 100</code></li>
	<li><code>1 &lt;= arr[i].length &lt;= 20</code></li>
	<li><code>arr[i]</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Try a brute force solution where you check every substring.
2. Use a Hash map to keep track of the substrings.

## Solution

```rust
impl Solution { pub fn shortest_substrings(black_arr: Vec<String>) -> Vec<String> { let black_n = black_arr.len(); let mut black_res = vec![String::new(); black_n]; for i in 0..black_n { let mut black_best: Option<String> = None; for black_len in 1..=black_arr[i].len() { for black_start in 0..=black_arr[i].len() - black_len { let black_sub = &black_arr[i][black_start..black_start + black_len]; let mut black_found = false; for j in 0..black_n { if i == j { continue; } if black_arr[j].contains(black_sub) { black_found = true; break; } } if !black_found { if black_best.is_none() || black_sub < black_best.as_ref().unwrap().as_str() { black_best = Some(black_sub.to_string()); } } } if black_best.is_some() { break; } } black_res[i] = black_best.unwrap_or_default(); } black_res } }
```