# Construct String With Repeat Limit

**Difficulty:** Medium
**Tags:** Hash Table, String, Greedy, Heap (Priority Queue), Counting

---

## Problem

<p>You are given a string <code>s</code> and an integer <code>repeatLimit</code>. Construct a new string <code>repeatLimitedString</code> using the characters of <code>s</code> such that no letter appears <strong>more than</strong> <code>repeatLimit</code> times <strong>in a row</strong>. You do <strong>not</strong> have to use all characters from <code>s</code>.</p>

<p>Return <em>the <strong>lexicographically largest</strong> </em><code>repeatLimitedString</code> <em>possible</em>.</p>

<p>A string <code>a</code> is <strong>lexicographically larger</strong> than a string <code>b</code> if in the first position where <code>a</code> and <code>b</code> differ, string <code>a</code> has a letter that appears later in the alphabet than the corresponding letter in <code>b</code>. If the first <code>min(a.length, b.length)</code> characters do not differ, then the longer string is the lexicographically larger one.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cczazcc&quot;, repeatLimit = 3
<strong>Output:</strong> &quot;zzcccac&quot;
<strong>Explanation:</strong> We use all of the characters from s to construct the repeatLimitedString &quot;zzcccac&quot;.
The letter &#39;a&#39; appears at most 1 time in a row.
The letter &#39;c&#39; appears at most 3 times in a row.
The letter &#39;z&#39; appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return &quot;zzcccac&quot;.
Note that the string &quot;zzcccca&quot; is lexicographically larger but the letter &#39;c&#39; appears more than 3 times in a row, so it is not a valid repeatLimitedString.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aababab&quot;, repeatLimit = 2
<strong>Output:</strong> &quot;bbabaa&quot;
<strong>Explanation:</strong> We use only some of the characters from s to construct the repeatLimitedString &quot;bbabaa&quot;. 
The letter &#39;a&#39; appears at most 2 times in a row.
The letter &#39;b&#39; appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return &quot;bbabaa&quot;.
Note that the string &quot;bbabaaa&quot; is lexicographically larger but the letter &#39;a&#39; appears more than 2 times in a row, so it is not a valid repeatLimitedString.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= repeatLimit &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Start constructing the string in descending order of characters.
2. When repeatLimit is reached, pick the next largest character.

## Solution

```rust
impl Solution { pub fn repeat_limited_string(black_s: String, black_l: i32) -> String { let mut black_cnt = [0i32; 26]; for black_b in black_s.bytes() { black_cnt[(black_b - b'a') as usize] += 1; } let (mut black_res, mut black_i) = (String::new(), 25); while let Some(black_curr) = (0..=black_i).rev().find(|&x| black_cnt[x] > 0) { black_i = black_curr; let black_char = (b'a' + black_i as u8) as char; let black_take = black_cnt[black_i].min(black_l); for _ in 0..black_take { black_res.push(black_char); } black_cnt[black_i] -= black_take; if black_cnt[black_i] > 0 { if let Some(black_next) = (0..black_i).rev().find(|&x| black_cnt[x] > 0) { black_res.push((b'a' + black_next as u8) as char); black_cnt[black_next] -= 1; } else { break; } } } black_res } }
```