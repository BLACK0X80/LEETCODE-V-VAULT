# Count Substrings That Can Be Rearranged to Contain a String I

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>You are given two strings <code>word1</code> and <code>word2</code>.</p>

<p>A string <code>x</code> is called <strong>valid</strong> if <code>x</code> can be rearranged to have <code>word2</code> as a <span data-keyword="string-prefix">prefix</span>.</p>

<p>Return the total number of <strong>valid</strong> <span data-keyword="substring-nonempty">substrings</span> of <code>word1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;bcca&quot;, word2 = &quot;abc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only valid substring is <code>&quot;bcca&quot;</code> which can be rearranged to <code>&quot;abcc&quot;</code> having <code>&quot;abc&quot;</code> as a prefix.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;abcabc&quot;, word2 = &quot;abc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>All the substrings except substrings of size 1 and size 2 are valid.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;abcabc&quot;, word2 = &quot;aaabc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word1.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= word2.length &lt;= 10<sup>4</sup></code></li>
	<li><code>word1</code> and <code>word2</code> consist only of lowercase English letters.</li>
</ul>


## Hints

1. Store the frequency of each character for all prefixes.
2. Use Binary Search.

## Solution

```rust
impl Solution { pub fn valid_substring_count(black_w1: String, black_w2: String) -> i64 { let (black_b1, black_b2) = (black_w1.as_bytes(), black_w2.as_bytes()); let (mut black_target, mut black_cur, mut black_l, mut black_ans) = (vec![0; 26], vec![0; 26], 0, 0i64); for &c in black_b2 { black_target[(c - b'a') as usize] += 1; } let black_check = |black_c: &[i32], black_t: &[i32]| (0..26).all(|i| black_c[i] >= black_t[i]); for black_r in 0..black_b1.len() { black_cur[(black_b1[black_r] - b'a') as usize] += 1; while black_check(&black_cur, &black_target) { black_ans += (black_b1.len() - black_r) as i64; black_cur[(black_b1[black_l] - b'a') as usize] -= 1; black_l += 1; } } black_ans } }
```