# Swap For Longest Repeated Character Substring

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>You are given a string <code>text</code>. You can swap two of the characters in the <code>text</code>.</p>

<p>Return <em>the length of the longest substring with repeated characters</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;ababa&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can swap the first &#39;b&#39; with the last &#39;a&#39;, or the last &#39;b&#39; with the first &#39;a&#39;. Then, the longest repeated character substring is &quot;aaa&quot; with length 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;aaabaaa&quot;
<strong>Output:</strong> 6
<strong>Explanation:</strong> Swap &#39;b&#39; with the last &#39;a&#39; (or the first &#39;a&#39;), and we get longest repeated character substring &quot;aaaaaa&quot; with length 6.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;aaaaa&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> No need to swap, longest repeated character substring is &quot;aaaaa&quot; with length is 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= text.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>text</code> consist of lowercase English characters only.</li>
</ul>


## Hints

1. There are two cases:  a block of characters, or two blocks of characters between one different character. 
 By keeping a run-length encoded version of the string, we can easily check these cases.

## Solution

```rust
impl Solution { pub fn max_rep_opt1(black_t: String) -> i32 { let (black_b, mut black_c, mut black_res) = (black_t.as_bytes(), [0; 26], 0); for &black_v in black_b { black_c[(black_v - b'a') as usize] += 1; } for black_i in 0..26 { let (mut black_l, mut black_cnt, mut black_diff, black_char) = (0, 0, 0, (black_i as u8 + b'a')); for black_r in 0..black_b.len() { if black_b[black_r] == black_char { black_cnt += 1; } else { black_diff += 1; } while black_diff > 1 { if black_b[black_l] == black_char { black_cnt -= 1; } else { black_diff -= 1; } black_l += 1; } black_res = black_res.max(black_cnt + (black_c[black_i] > black_cnt) as i32); } } black_res } }
```