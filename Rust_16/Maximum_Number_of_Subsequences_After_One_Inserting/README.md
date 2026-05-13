# Maximum Number of Subsequences After One Inserting

**Difficulty:** Medium
**Tags:** String, Dynamic Programming, Greedy, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> consisting of uppercase English letters.</p>

<p>You are allowed to insert <strong>at most one</strong> uppercase English letter at <strong>any</strong> position (including the beginning or end) of the string.</p>

<p>Return the <strong>maximum</strong> number of <code>&quot;LCT&quot;</code> <span data-keyword="subsequence-string-nonempty">subsequences</span> that can be formed in the resulting string after <strong>at most one insertion</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;LMCT&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We can insert a <code>&quot;L&quot;</code> at the beginning of the string s to make <code>&quot;LLMCT&quot;</code>, which has 2 subsequences, at indices [0, 3, 4] and [1, 3, 4].</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;LCCT&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>We can insert a <code>&quot;L&quot;</code> at the beginning of the string s to make <code>&quot;LLCCT&quot;</code>, which has 4 subsequences, at indices [0, 2, 4], [0, 3, 4], [1, 2, 4] and [1, 3, 4].</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;L&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>Since it is not possible to obtain the subsequence <code>&quot;LCT&quot;</code> by inserting a single letter, the result is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of uppercase English letters.</li>
</ul>


## Hints

1. Precompute <code>preL</code>, <code>preLC</code>, <code>sufT</code>, and <code>sufCT</code> arrays to count L’s, LC’s, T’s, and CT’s at each position.
2. Compute <code>base</code> as the sum over all i of <code>preLC[i] * sufT[i]</code>.
3. For each insert position i, compute gains <code>sufCT[i]</code> for ‘L’, <code>preL[i] * sufT[i]</code> for ‘C’, and <code>preLC[i]</code> for ‘T’, and take the maximum of <code>base</code> and <code>base + gain</code>.

## Solution

```rust
impl Solution { pub fn num_of_subsequences(black_s: String) -> i64 { let black_b = black_s.as_bytes(); let black_n = black_b.len(); let (mut black_l, mut black_lc, mut black_lct) = (0i64, 0i64, 0i64); let (mut black_pre_l, mut black_suf_t) = (vec![0i64; black_n + 1], vec![0i64; black_n + 1]); for black_i in 0..black_n { black_pre_l[black_i + 1] = black_pre_l[black_i] + if black_b[black_i] == b'L' {1} else {0}; if black_b[black_i] == b'L' { black_l += 1; } else if black_b[black_i] == b'C' { black_lc += black_l; } else if black_b[black_i] == b'T' { black_lct += black_lc; } } let (mut black_t_cnt, mut black_ct) = (0i64, 0i64); for black_i in (0..black_n).rev() { black_suf_t[black_i] = black_suf_t[black_i + 1] + if black_b[black_i] == b'T' {1} else {0}; if black_b[black_i] == b'T' { black_t_cnt += 1; } else if black_b[black_i] == b'C' { black_ct += black_t_cnt; } } let mut black_best = black_lc.max(black_ct); for black_i in 0..=black_n { black_best = black_best.max(black_pre_l[black_i] * black_suf_t[black_i]); } black_lct + black_best } }
```