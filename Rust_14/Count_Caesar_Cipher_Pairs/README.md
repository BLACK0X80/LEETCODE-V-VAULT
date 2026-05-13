# Count Caesar Cipher Pairs

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, String, Counting

---

## Problem

<p>You are given an array <code>words</code> of <code>n</code> strings. Each string has length <code>m</code> and contains only lowercase English letters.</p>

<p>Two strings <code>s</code> and <code>t</code> are <strong>similar</strong> if we can apply the following operation any number of times (possibly zero times) so that <code>s</code> and <code>t</code> become <strong>equal</strong>.</p>

<ul>
	<li>Choose either <code>s</code> or <code>t</code>.</li>
	<li>Replace <strong>every</strong> letter in the chosen string with the next letter in the alphabet cyclically. The next letter after <code>&#39;z&#39;</code> is <code>&#39;a&#39;</code>.</li>
</ul>

<p>Count the number of pairs of indices <code>(i, j)</code> such that:</p>

<ul>
	<li><code>i &lt; j</code></li>
	<li><code>words[i]</code> and <code>words[j]</code> are <strong>similar</strong>.</li>
</ul>

<p>Return an integer denoting the number of such pairs.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;fusion&quot;,&quot;layout&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><code>words[0] = &quot;fusion&quot;</code> and <code>words[1] = &quot;layout&quot;</code> are similar because we can apply the operation to <code>&quot;fusion&quot;</code> 6 times. The string <code>&quot;fusion&quot;</code> changes as follows.</p>

<ul>
	<li><code>&quot;fusion&quot;</code></li>
	<li><code>&quot;gvtjpo&quot;</code></li>
	<li><code>&quot;hwukqp&quot;</code></li>
	<li><code>&quot;ixvlrq&quot;</code></li>
	<li><code>&quot;jywmsr&quot;</code></li>
	<li><code>&quot;kzxnts&quot;</code></li>
	<li><code>&quot;layout&quot;</code></li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;ab&quot;,&quot;aa&quot;,&quot;za&quot;,&quot;aa&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><code>words[0] = &quot;ab&quot;</code> and <code>words[2] = &quot;za&quot;</code> are similar. <code>words[1] = &quot;aa&quot;</code> and <code>words[3] = &quot;aa&quot;</code> are similar.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == words.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= m == words[i].length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= n * m &lt;= 10<sup>5</sup></code></li>
	<li><code>words[i]</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Two strings are similar if the differences between consecutive characters (mod 26) are the same; normalize each string by shifting its first character to <code>'a'</code>.
2. Compute a hashable key for each normalized string representing its relative character differences.
3. Use a map to count how many strings share the same normalized key.

## Solution

```rust
use std::collections::HashMap; impl Solution { pub fn count_pairs(black_words: Vec<String>) -> i64 { let mut black_m = HashMap::new(); for black_s in black_words { let black_b = black_s.as_bytes(); let black_k: Vec<i32> = (1..black_b.len()).map(|black_i| (black_b[black_i] as i32 - black_b[0] as i32 + 26) % 26).collect(); *black_m.entry(black_k).or_insert(0_i64) += 1; } black_m.values().map(|&black_v| black_v * (black_v - 1) / 2).sum() } }
```