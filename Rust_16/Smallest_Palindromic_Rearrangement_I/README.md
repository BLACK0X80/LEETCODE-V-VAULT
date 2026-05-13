# Smallest Palindromic Rearrangement I

**Difficulty:** Medium
**Tags:** String, Sorting, Counting Sort

---

## Problem

<p>You are given a <strong><span data-keyword="palindrome-string">palindromic</span></strong> string <code>s</code>.</p>

<p>Return the <strong><span data-keyword="lexicographically-smaller-string">lexicographically smallest</span></strong> palindromic <span data-keyword="permutation-string">permutation</span> of <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;z&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;z&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>A string of only one character is already the lexicographically smallest palindrome.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;babab&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;abbba&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>Rearranging <code>&quot;babab&quot;</code> &rarr; <code>&quot;abbba&quot;</code> gives the smallest lexicographic palindrome.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;daccad&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;acddca&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>Rearranging <code>&quot;daccad&quot;</code> &rarr; <code>&quot;acddca&quot;</code> gives the smallest lexicographic palindrome.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
	<li><code>s</code> is guaranteed to be palindromic.</li>
</ul>


## Hints

1. Consider a palindrome as composed of two mirror-image halves.
2. Construct one half (using <code>s</code>), and then the other half is its reverse to obtain the lexicographically smallest permutation.

## Solution

```rust
impl Solution { pub fn smallest_palindrome(s: String) -> String { let mut black_cnt = [0; 26]; for black_b in s.bytes() { black_cnt[(black_b - b'a') as usize] += 1; } let (mut black_mid, mut black_half) = (String::new(), vec![]); for black_i in 0..26 { if black_cnt[black_i] % 2 == 1 { black_mid.push((b'a' + black_i as u8) as char); } for _ in 0..black_cnt[black_i]/2 { black_half.push((b'a' + black_i as u8) as char); } } let black_f: String = black_half.iter().collect(); let black_l: String = black_half.iter().rev().collect(); format!("{}{}{}", black_f, black_mid, black_l) } }
```