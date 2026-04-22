# Reorganize String

**Difficulty:** Medium
**Tags:** Hash Table, String, Greedy, Sorting, Heap (Priority Queue), Counting

---

## Problem

<p>Given a string <code>s</code>, rearrange the characters of <code>s</code> so that any two adjacent characters are not the same.</p>

<p>Return <em>any possible rearrangement of</em> <code>s</code> <em>or return</em> <code>&quot;&quot;</code> <em>if not possible</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> s = "aab"
<strong>Output:</strong> "aba"
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> s = "aaab"
<strong>Output:</strong> ""
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 500</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Alternate placing the most common letters.

## Solution

```rust
impl Solution { pub fn reorganize_string(black_s: String) -> String { let mut black_cnt = [0; 26]; for b in black_s.bytes() { black_cnt[(b - b'a') as usize] += 1; } let (mut black_max_f, mut black_char) = (0, 0); for i in 0..26 { if black_cnt[i] > black_max_f { black_max_f = black_cnt[i]; black_char = i; } } if black_max_f > (black_s.len() + 1) / 2 { return "".to_string(); } let mut black_res = vec![0u8; black_s.len()]; let mut i = 0; while black_cnt[black_char] > 0 { black_res[i] = black_char as u8 + b'a'; i += 2; black_cnt[black_char] -= 1; } for c in 0..26 { while black_cnt[c] > 0 { if i >= black_s.len() { i = 1; } black_res[i] = c as u8 + b'a'; i += 2; black_cnt[c] -= 1; } } String::from_utf8(black_res).unwrap() } }
```