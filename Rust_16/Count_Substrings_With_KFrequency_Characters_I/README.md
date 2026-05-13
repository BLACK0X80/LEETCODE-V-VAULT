# Count Substrings With K-Frequency Characters I

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>Given a string <code>s</code> and an integer <code>k</code>, return the total number of <span data-keyword="substring-nonempty">substrings</span> of <code>s</code> where <strong>at least one</strong> character appears <strong>at least</strong> <code>k</code> times.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abacb&quot;, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The valid substrings are:</p>

<ul>
	<li><code>&quot;aba&quot;</code> (character <code>&#39;a&#39;</code> appears 2 times).</li>
	<li><code>&quot;abac&quot;</code> (character <code>&#39;a&#39;</code> appears 2 times).</li>
	<li><code>&quot;abacb&quot;</code> (character <code>&#39;a&#39;</code> appears 2 times).</li>
	<li><code>&quot;bacb&quot;</code> (character <code>&#39;b&#39;</code> appears 2 times).</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abcde&quot;, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">15</span></p>

<p><strong>Explanation:</strong></p>

<p>All substrings are valid because every character appears at least once.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 3000</code></li>
	<li><code>1 &lt;= k &lt;= s.length</code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Fix the <code>left</code> index of the substring.
2. For the fixed <code>left</code> index, find the first <code>right</code> index for which substring <code>s[left..right]</code> satisfies the condition.
3. Every substring that starts at <code>left</code> and ends after <code>right</code> satisfies the condition.

## Solution

```rust
impl Solution { pub fn number_of_substrings(black_s: String, black_k: i32) -> i32 { let (black_b, mut black_l, mut black_ans, mut black_cnt) = (black_s.as_bytes(), 0, 0, vec![0; 26]); for black_r in 0..black_b.len() { black_cnt[(black_b[black_r] - b'a') as usize] += 1; while black_cnt.iter().any(|&x| x >= black_k) { black_ans += (black_b.len() - black_r) as i32; black_cnt[(black_b[black_l] - b'a') as usize] -= 1; black_l += 1; } } black_ans } }
```