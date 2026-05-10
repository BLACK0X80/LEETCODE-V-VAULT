# Count of Substrings Containing Every Vowel and K Consonants II

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>You are given a string <code>word</code> and a <strong>non-negative</strong> integer <code>k</code>.</p>

<p>Return the total number of <span data-keyword="substring-nonempty">substrings</span> of <code>word</code> that contain every vowel (<code>&#39;a&#39;</code>, <code>&#39;e&#39;</code>, <code>&#39;i&#39;</code>, <code>&#39;o&#39;</code>, and <code>&#39;u&#39;</code>) <strong>at least</strong> once and <strong>exactly</strong> <code>k</code> consonants.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;aeioqq&quot;, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>There is no substring with every vowel.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;aeiou&quot;, k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only substring with every vowel and zero consonants is <code>word[0..4]</code>, which is <code>&quot;aeiou&quot;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;</span>ieaouqqieaouqq<span class="example-io">&quot;, k = 1</span></p>

<p><strong>Output:</strong> 3</p>

<p><strong>Explanation:</strong></p>

<p>The substrings with every vowel and one consonant are:</p>

<ul>
	<li><code>word[0..5]</code>, which is <code>&quot;ieaouq&quot;</code>.</li>
	<li><code>word[6..11]</code>, which is <code>&quot;qieaou&quot;</code>.</li>
	<li><code>word[7..12]</code>, which is <code>&quot;ieaouq&quot;</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>5 &lt;= word.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>word</code> consists only of lowercase English letters.</li>
	<li><code>0 &lt;= k &lt;= word.length - 5</code></li>
</ul>


## Hints

1. We can use sliding window and binary search.
2. For each index <code>r</code>, find the maximum <code>l</code> such that both conditions are satisfied using binary search.

## Solution

```rust
impl Solution { pub fn count_of_substrings(black_w: String, black_k: i32) -> i64 { fn black_at_least(black_s: &[u8], black_k: i32) -> i64 { if black_k < 0 { return 0; } let (mut black_l, mut black_ans, mut black_cons, mut black_v) = (0, 0i64, 0, std::collections::HashMap::new()); let black_is_v = |black_c: u8| b"aeiou".contains(&black_c); for black_r in 0..black_s.len() { if black_is_v(black_s[black_r]) { *black_v.entry(black_s[black_r]).or_insert(0) += 1; } else { black_cons += 1; } while black_v.len() == 5 && black_cons >= black_k { black_ans += (black_s.len() - black_r) as i64; if black_is_v(black_s[black_l]) { let black_entry = black_v.get_mut(&black_s[black_l]).unwrap(); *black_entry -= 1; if *black_entry == 0 { black_v.remove(&black_s[black_l]); } } else { black_cons -= 1; } black_l += 1; } } black_ans } let black_b = black_w.as_bytes(); black_at_least(black_b, black_k) - black_at_least(black_b, black_k + 1) } }
```