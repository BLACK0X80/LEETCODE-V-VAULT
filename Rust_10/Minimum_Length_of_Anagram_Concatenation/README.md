# Minimum Length of Anagram Concatenation

**Difficulty:** Medium
**Tags:** Hash Table, String, Counting

---

## Problem

<p>You are given a string <code>s</code>, which is known to be a concatenation of <strong>anagrams</strong> of some string <code>t</code>.</p>

<p>Return the <strong>minimum</strong> possible length of the string <code>t</code>.</p>

<p>An <strong>anagram</strong> is formed by rearranging the letters of a string. For example, &quot;aab&quot;, &quot;aba&quot;, and, &quot;baa&quot; are anagrams of &quot;aab&quot;.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>One possible string <code>t</code> could be <code>&quot;ba&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;cdef&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>One possible string <code>t</code> could be <code>&quot;cdef&quot;</code>, notice that <code>t</code> can be equal to <code>s</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abcbcacabbaccba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consist only of lowercase English letters.</li>
</ul>


## Hints

1. The answer should be a divisor of <code>s.length</code>.
2. Check each candidate naively.

## Solution

```rust
impl Solution { pub fn min_anagram_length(black_s: String) -> i32 { let (black_n, black_b) = (black_s.len(), black_s.as_bytes()); let mut black_total_cnt = [0; 26]; for &b in black_b { black_total_cnt[(b - b'a') as usize] += 1; } for black_len in 1..=black_n { if black_n % black_len != 0 { continue; } let mut black_base = [0; 26]; for i in 0..black_len { black_base[(black_b[i] - b'a') as usize] += 1; } let mut black_ok = true; for i in (0..black_n).step_by(black_len) { let mut black_curr = [0; 26]; for j in 0..black_len { black_curr[(black_b[i+j] - b'a') as usize] += 1; } if black_curr != black_base { black_ok = false; break; } } if black_ok { return black_len as i32; } } black_n as i32 } }
```