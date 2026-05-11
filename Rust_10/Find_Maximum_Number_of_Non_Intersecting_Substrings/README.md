# Find Maximum Number of Non Intersecting Substrings

**Difficulty:** Medium
**Tags:** Hash Table, String, Dynamic Programming, Greedy

---

## Problem

<p>You are given a string <code>word</code>.</p>

<p>Return the <strong>maximum</strong> number of non-intersecting <strong><span data-keyword="substring-nonempty">substrings</span></strong> of word that are at <strong>least</strong> four characters long and start and end with the same letter.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;abcdeafdef&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The two substrings are <code>&quot;abcdea&quot;</code> and <code>&quot;fdef&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;bcdaaaab&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only substring is <code>&quot;aaaa&quot;</code>. Note that we cannot <strong>also</strong> choose <code>&quot;bcdaaaab&quot;</code> since it intersects with the other substring.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>word</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Can we solve the problem using Dynamic Programming?
2. For each character <code>c</code>, store all occurrence indices in order
3. At each position <code>i</code>, let <code>j</code> be the first index of <code>word[i]</code>; if <code>i - j >= 3</code>, we can form substring <code>[j, i]</code>
4. For each index, also store the maximum for <b>any</b> substring ending before that index in the dp.

## Solution

```rust
use std::collections::{HashMap, VecDeque}; impl Solution { pub fn max_substrings(black_s: String) -> i32 { let black_b = black_s.as_bytes(); let black_n = black_b.len(); let mut black_dp = vec![0; black_n + 1]; let mut black_pos: HashMap<u8, VecDeque<usize>> = HashMap::new(); for black_i in 0..black_n { let black_c = black_b[black_i]; black_dp[black_i + 1] = black_dp[black_i]; if let Some(black_deq) = black_pos.get(&black_c) { for &black_j in black_deq { if black_i - black_j + 1 >= 4 { black_dp[black_i + 1] = black_dp[black_i + 1].max(black_dp[black_j] + 1); } } } black_pos.entry(black_c).or_insert(VecDeque::new()).push_back(black_i); if black_pos.get(&black_c).unwrap().len() > 4 { black_pos.get_mut(&black_c).unwrap().pop_front(); } } black_dp[black_n] } }
```