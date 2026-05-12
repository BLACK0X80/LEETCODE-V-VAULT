# Longest Balanced Substring II

**Difficulty:** Medium
**Tags:** Hash Table, String, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> consisting only of the characters <code>&#39;a&#39;</code>, <code>&#39;b&#39;</code>, and <code>&#39;c&#39;</code>.</p>

<p>A <strong><span data-keyword="substring-nonempty">substring</span></strong> of <code>s</code> is called <strong>balanced</strong> if all <strong>distinct</strong> characters in the <strong>substring</strong> appear the <strong>same</strong> number of times.</p>

<p>Return the <strong>length of the longest balanced substring</strong> of <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abbac&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The longest balanced substring is <code>&quot;abba&quot;</code> because both distinct characters <code>&#39;a&#39;</code> and <code>&#39;b&#39;</code> each appear exactly 2 times.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;aabcc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The longest balanced substring is <code>&quot;abc&quot;</code> because all distinct characters <code>&#39;a&#39;</code>, <code>&#39;b&#39;</code> and <code>&#39;c&#39;</code> each appear exactly 1 time.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;aba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>One of the longest balanced substrings is <code>&quot;ab&quot;</code> because both distinct characters <code>&#39;a&#39;</code> and <code>&#39;b&#39;</code> each appear exactly 1 time. Another longest balanced substring is <code>&quot;ba&quot;</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> contains only the characters <code>&#39;a&#39;</code>, <code>&#39;b&#39;</code>, and <code>&#39;c&#39;</code>.</li>
</ul>


## Hints

1. Solve for three cases: all-equal characters, exactly two distinct characters, and all three characters present. Treat each case separately and take the maximum length.
2. Case 1: single character: the longest balanced substring is the longest run of the same character; report its length.
3. Case 2: two distinct characters: reduce to that pair (ignore the third character) and use prefix differences of their counts; equal counts between two indices mean the substring between them is balanced for those two chars.
4. Case 3: all three characters: use prefix counts and hash the pair <code>(count_b - count_a, count_c - count_a)</code> for each prefix; if the same pair appears at two indices the substring between them has equal counts for a, b, and c. Store earliest index per pair to get maximal length.

## Solution

```rust
use std::collections::HashMap; impl Solution { pub fn longest_balanced(black_s: String) -> i32 { let black_b = black_s.as_bytes(); let black_mono = || { let (mut black_a, mut black_c) = (0, 0); for black_i in 0..black_b.len() { if black_i > 0 && black_b[black_i] == black_b[black_i-1] { black_c += 1; } else { black_c = 1; } black_a = black_a.max(black_c); } black_a }; let black_duo = |black_c1: u8, black_c2: u8| { let (mut black_a, mut black_d, mut black_p) = (0, 0, HashMap::new()); black_p.insert(0, -1); for black_i in 0..black_b.len() as i32 { if black_b[black_i as usize] != black_c1 && black_b[black_i as usize] != black_c2 { black_p.clear(); black_p.insert(0, black_i); black_d = 0; continue; } if black_b[black_i as usize] == black_c1 { black_d += 1; } else { black_d -= 1; } if let Some(&black_prev) = black_p.get(&black_d) { black_a = black_a.max(black_i - black_prev); } else { black_p.insert(black_d, black_i); } } black_a }; let black_trio = || { let (mut black_a, mut black_cnt, mut black_p) = (0, vec![0; 3], HashMap::new()); black_p.insert(vec![0, 0], -1); for black_i in 0..black_b.len() as i32 { black_cnt[(black_b[black_i as usize] - b'a') as usize] += 1; let black_key = vec![black_cnt[1] - black_cnt[0], black_cnt[2] - black_cnt[0]]; if let Some(&black_prev) = black_p.get(&black_key) { black_a = black_a.max(black_i - black_prev); } else { black_p.insert(black_key, black_i); } } black_a }; black_mono().max(black_duo(b'a', b'b')).max(black_duo(b'a', b'c')).max(black_duo(b'b', b'c')).max(black_trio()) } }
```