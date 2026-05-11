# Count Distinct Integers After Removing Zeros

**Difficulty:** Medium
**Tags:** Math, Dynamic Programming

---

## Problem

<p>You are given a <strong>positive</strong> integer <code>n</code>.</p>

<p>For every integer <code>x</code> from 1 to <code>n</code>, we write down the integer obtained by removing all zeros from the decimal representation of <code>x</code>.</p>

<p>Return an integer denoting the number of <strong>distinct</strong> integers written down.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<p>The integers we wrote down are 1, 2, 3, 4, 5, 6, 7, 8, 9, 1. There are 9 distinct integers (1, 2, 3, 4, 5, 6, 7, 8, 9).</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The integers we wrote down are 1, 2, 3. There are 3 distinct integers (1, 2, 3).</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Build integers less than or equal to <code>n</code> using only digits from 1 to 9
2. Count such integers using math or digit DP

## Solution

```rust
impl Solution { pub fn count_distinct(black_n: i64) -> i64 { let black_s = black_n.to_string(); let black_l = black_s.len(); let mut black_m = std::collections::HashMap::new(); fn black_dp(black_p: usize, black_t: bool, black_s_st: bool, black_len: usize, black_str: &str, black_memo: &mut std::collections::HashMap<(usize, bool, bool), i64>) -> i64 { if black_p == black_len { return if black_s_st { 1 } else { 0 }; } if let Some(&v) = black_memo.get(&(black_p, black_t, black_s_st)) { return v; } let black_lim = if black_t { (black_str.as_bytes()[black_p] - b'0') as i32 } else { 9 }; let mut black_res = 0; for d in 0..=black_lim { if d == 0 { if !black_s_st { black_res += black_dp(black_p + 1, black_t && (d == black_lim), false, black_len, black_str, black_memo); } } else { black_res += black_dp(black_p + 1, black_t && (d == black_lim), true, black_len, black_str, black_memo); } } black_memo.insert((black_p, black_t, black_s_st), black_res); black_res } black_dp(0, true, false, black_l, &black_s, &mut black_m) } }
```