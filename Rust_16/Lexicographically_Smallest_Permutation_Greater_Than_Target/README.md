# Lexicographically Smallest Permutation Greater Than Target

**Difficulty:** Medium
**Tags:** Hash Table, String, Greedy, Counting, Enumeration

---

## Problem

<p>You are given two strings <code>s</code> and <code>target</code>, both having length <code>n</code>, consisting of lowercase English letters.</p>

<p>Return the <strong>lexicographically smallest <span data-keyword="permutation-string">permutation</span></strong> of <code>s</code> that is <strong>strictly</strong> greater than <code>target</code>. If no permutation of <code>s</code> is lexicographically strictly greater than <code>target</code>, return an empty string.</p>

<p>A string <code>a</code> is <strong>lexicographically strictly greater </strong>than a string <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, string <code>a</code> has a letter that appears later in the alphabet than the corresponding letter in <code>b</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abc&quot;, target = &quot;bba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;bca&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The permutations of <code>s</code> (in lexicographical order) are <code>&quot;abc&quot;</code>, <code>&quot;acb&quot;</code>, <code>&quot;bac&quot;</code>, <code>&quot;bca&quot;</code>, <code>&quot;cab&quot;</code>, and <code>&quot;cba&quot;</code>.</li>
	<li>The lexicographically smallest permutation that is strictly greater than <code>target</code> is <code>&quot;bca&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;leet&quot;, target = &quot;code&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;eelt&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The permutations of <code>s</code> (in lexicographical order) are <code>&quot;eelt&quot;</code>, <code>&quot;eetl&quot;</code>, <code>&quot;elet&quot;</code>, <code>&quot;elte&quot;</code>, <code>&quot;etel&quot;</code>, <code>&quot;etle&quot;</code>, <code>&quot;leet&quot;</code>, <code>&quot;lete&quot;</code>, <code>&quot;ltee&quot;</code>, <code>&quot;teel&quot;</code>, <code>&quot;tele&quot;</code>, and <code>&quot;tlee&quot;</code>.</li>
	<li>The lexicographically smallest permutation that is strictly greater than <code>target</code> is <code>&quot;eelt&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;baba&quot;, target = &quot;bbaa&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The permutations of <code>s</code> (in lexicographical order) are <code>&quot;aabb&quot;</code>, <code>&quot;abab&quot;</code>, <code>&quot;abba&quot;</code>, <code>&quot;baab&quot;</code>, <code>&quot;baba&quot;</code>, and <code>&quot;bbaa&quot;</code>.</li>
	<li>None of them is lexicographically strictly greater than <code>target</code>. Therefore, the answer is <code>&quot;&quot;</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length == target.length &lt;= 300</code></li>
	<li><code>s</code> and <code>target</code> consist of only lowercase English letters.</li>
</ul>


## Hints

1. Maintain frequency counts of <code>s</code>.
2. Walk left-to-right; if equal to <code>target[i]</code> is possible, take it and continue.
3. If not, try the smallest letter strictly greater than <code>target[i]</code>.
4. If neither, backtrack left to the most recent index where you matched <code>target</code> and try to bump there.

## Solution

```rust
impl Solution { pub fn lex_greater_permutation(black_s: String, black_t: String) -> String { let (mut black_cnt, black_n) = (vec![0; 26], black_s.len()); for black_b in black_s.bytes() { black_cnt[(black_b - b'a') as usize] += 1; } let mut black_res = vec![0u8; black_n]; for black_i in (0..black_n).rev() { let mut black_cur_cnt = vec![0; 26]; let mut black_possible = true; for black_j in 0..black_i { let black_idx = (black_t.as_bytes()[black_j] - b'a') as usize; black_cur_cnt[black_idx] += 1; if black_cur_cnt[black_idx] > black_cnt[black_idx] { black_possible = false; break; } } if !black_possible { continue; } for black_c in (black_t.as_bytes()[black_i] - b'a' + 1)..26 { if black_cur_cnt[black_c as usize] < black_cnt[black_c as usize] { black_res[..black_i].copy_from_slice(&black_t.as_bytes()[..black_i]); black_res[black_i] = b'a' + black_c as u8; black_cur_cnt[black_c as usize] += 1; let mut black_ptr = black_i + 1; for black_ch in 0..26 { while black_cur_cnt[black_ch] < black_cnt[black_ch] { black_res[black_ptr] = b'a' + black_ch as u8; black_ptr += 1; black_cur_cnt[black_ch] += 1; } } return String::from_utf8(black_res).unwrap(); } } } "".to_string() } }
```