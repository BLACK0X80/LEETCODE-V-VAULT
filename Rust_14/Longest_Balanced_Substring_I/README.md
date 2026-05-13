# Longest Balanced Substring I

**Difficulty:** Medium
**Tags:** Hash Table, String, Counting, Enumeration

---

## Problem

<p>You are given a string <code>s</code> consisting of lowercase English letters.</p>

<p>A <strong><span data-keyword="substring-nonempty">substring</span></strong> of <code>s</code> is called <strong>balanced</strong> if all <strong>distinct</strong> characters in the <strong>substring</strong> appear the <strong>same</strong> number of times.</p>

<p>Return the <strong>length</strong> of the <strong>longest balanced substring</strong> of <code>s</code>.</p>

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
<p><strong>Input:</strong> <span class="example-io">s = &quot;zzabccy&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The longest balanced substring is <code>&quot;zabc&quot;</code> because the distinct characters <code>&#39;z&#39;</code>, <code>&#39;a&#39;</code>, <code>&#39;b&#39;</code>, and <code>&#39;c&#39;</code> each appear exactly 1 time.​​​​​​​</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;aba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><strong>​​​​​​​</strong>One of the longest balanced substrings is <code>&quot;ab&quot;</code> because both distinct characters <code>&#39;a&#39;</code> and <code>&#39;b&#39;</code> each appear exactly 1 time. Another longest balanced substring is <code>&quot;ba&quot;</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Use bruteforce over all substrings

## Solution

```rust
impl Solution { pub fn longest_balanced(black_s: String) -> i32 { let (black_b, black_n, mut black_ans) = (black_s.as_bytes(), black_s.len(), 0); for black_i in 0..black_n { let mut black_cnt = [0; 26]; for black_j in black_i..black_n { black_cnt[(black_b[black_j] - b'a') as usize] += 1; let (mut black_req, mut black_ok) = (0, true); for &black_v in &black_cnt { if black_v > 0 { if black_req == 0 { black_req = black_v; } else if black_v != black_req { black_ok = false; break; } } } if black_ok { black_ans = black_ans.max((black_j - black_i + 1) as i32); } } } black_ans } }
```