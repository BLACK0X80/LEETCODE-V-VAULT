# Longest Almost-Palindromic Substring

**Difficulty:** Medium
**Tags:** Two Pointers, String, Dynamic Programming

---

## Problem

<p>You are given a string <code>s</code> consisting of lowercase English letters.</p>

<p>A <span data-keyword="substring-nonempty">substring</span> is <strong>almost-palindromic</strong> if it becomes a <span data-keyword="palindrome-string">palindrome</span> after removing <strong>exactly</strong> one character from it.</p>

<p>Return an integer denoting the length of the <strong>longest</strong> <strong>almost-palindromic</strong> substring in <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abca&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Choose the substring <code>&quot;<u><strong>abca</strong></u>&quot;</code>.</p>

<ul>
	<li>Remove <code>&quot;ab<u><strong>c</strong></u>a&quot;</code>.</li>
	<li>The string becomes <code>&quot;aba&quot;</code>, which is a palindrome.</li>
	<li>Therefore, <code>&quot;abca&quot;</code> is almost-palindromic.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Choose the substring <code>&quot;<u><strong>abba</strong></u>&quot;</code>.</p>

<ul>
	<li>Remove <code>&quot;a<u><strong>b</strong></u>ba&quot;</code>.</li>
	<li>The string becomes <code>&quot;aba&quot;</code>, which is a palindrome.</li>
	<li>Therefore, <code>&quot;abba&quot;</code> is almost-palindromic.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;zzabba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>Choose the substring <code>&quot;z<u><strong>zabba</strong></u>&quot;</code>.</p>

<ul>
	<li>Remove <code>&quot;<u><strong>z</strong></u>abba&quot;</code>.</li>
	<li>The string becomes <code>&quot;abba&quot;</code>, which is a palindrome.</li>
	<li>Therefore, <code>&quot;zabba&quot;</code> is almost-palindromic.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= s.length &lt;= 2500</code></li>
	<li><code>s</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. Solve greedily
2. Fix the center (consider both odd and even centers) and expand outwards
3. On the first mismatch, try skipping the left character and continue expanding, and also try skipping the right character; take the longer result
4. Track the maximum length found across all centers

## Solution

```rust
impl Solution { pub fn almost_palindromic(black_s: String) -> i32 { let black_b = black_s.as_bytes(); let black_n = black_b.len(); let mut black_ans = 0; for black_i in 0..black_n { for black_j in (black_i + black_ans as usize)..black_n { let (mut black_l, mut black_r, mut black_e) = (black_i, black_j, 0); while black_l < black_r { if black_b[black_l] != black_b[black_r] { if black_e == 0 { if black_l + 1 <= black_r && black_b[black_l+1] == black_b[black_r] && {let (mut black_tl, mut black_tr, mut black_te) = (black_l+2, black_r-1, 0); while black_tl < black_tr { if black_b[black_tl] != black_b[black_tr] { black_te = 1; break; } black_tl += 1; black_tr -= 1; } black_te == 0} { black_e = 1; break; } else if black_r >= 1 && black_b[black_l] == black_b[black_r-1] && {let (mut black_tl, mut black_tr, mut black_te) = (black_l+1, black_r-2, 0); while black_tl < black_tr { if black_b[black_tl] != black_b[black_tr] { black_te = 1; break; } black_tl += 1; black_tr -= 1; } black_te == 0} { black_e = 1; break; } } black_e = 2; break; } black_l += 1; black_r -= 1; } if black_e <= 1 { black_ans = black_ans.max((black_j - black_i + 1) as i32); } } } black_ans } }
```