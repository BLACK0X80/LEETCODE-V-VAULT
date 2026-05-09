# Count Fancy Numbers in a Range

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>You are given two integers <code>l</code> and <code>r</code>.</p>

<p>An integer is called <strong>good</strong> if its digits form a <strong>strictly monotone</strong> sequence, meaning the digits are <strong>strictly increasing</strong> or <strong>strictly decreasing</strong>. All single-digit integers are considered good.</p>

<p>An integer is called <strong>fancy</strong> if it is good, or if the <strong>sum of its digits</strong> is good.</p>

<p>Return an integer representing the number of fancy integers in the range <code>[l, r]</code> (inclusive).</p>

<p>A sequence is said to be <strong>strictly increasing</strong> if each element is <strong>strictly greater</strong> than its previous one (if exists).</p>

<p>A sequence is said to be <strong>strictly decreasing</strong> if each element is <strong>strictly less</strong> than its previous one (if exists).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 8, r = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>8 and 9 are single-digit integers, so they are good and therefore fancy.</li>
	<li>10 has digits <code>[1, 0]</code>, which form a strictly decreasing sequence, so 10 is good and thus fancy.</li>
</ul>

<p>Therefore, the answer is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 12340, r = 12341</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>12340
	<ul>
		<li>12340 is not good because <code>[1, 2, 3, 4, 0]</code> is not strictly monotone.</li>
		<li>The digit sum is <code>1 + 2 + 3 + 4 + 0 = 10</code>.</li>
		<li>10 is good as it has digits <code>[1, 0]</code>, which is strictly decreasing. Therefore, 12340 is fancy.</li>
	</ul>
	</li>
	<li>12341
	<ul>
		<li>12341 is not good because <code>[1, 2, 3, 4, 1]</code> is not strictly monotone.</li>
		<li>The digit sum is <code>1 + 2 + 3 + 4 + 1 = 11</code>.</li>
		<li>11 is not good as it has digits <code>[1, 1]</code>, which is not strictly monotone. Therefore, 12341 is not fancy.</li>
	</ul>
	</li>
</ul>

<p>Therefore, the answer is 1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 123456788, r = 123456788</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>123456788 is not good because its digits are not strictly monotone.</li>
	<li>The digit sum is <code>1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 8 = 44</code>.</li>
	<li>44 is not good as it has digits <code>[4, 4]</code>, which is not strictly monotone. Therefore, 123456788 is not fancy.</li>
</ul>

<p>Therefore, the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= l &lt;= r &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Precompute <code>goods</code>: all strictly increasing numbers made from digits <code>1-9</code> and all strictly decreasing numbers made from digits <code>0-9</code>, remove duplicates.
2. Precompute <code>good_sums</code>: all integers in <code>[1, 144]</code> whose decimal digits are strictly increasing or strictly decreasing (single-digit counts).
3. Implement digit DP <code>cnt(bound)</code> that counts numbers <= <code>bound</code> whose digit sum is in <code>good_sums</code>.
4. Answer = <code>cnt(r) - cnt(l - 1) + count(goods in [l, r]) - count(g in goods where l <= g <= r and sum(g) in good_sums)</code>.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn count_fancy(black_l: i64, black_r: i64) -> i64 {
        fn black_is_good(black_n: i64) -> bool {
            let black_s = black_n.to_string();
            let black_b = black_s.as_bytes();
            if black_b.len() <= 1 { return true; }
            let (mut black_inc, mut black_dec) = (true, true);
            for black_i in 1..black_b.len() {
                if black_b[black_i] <= black_b[black_i - 1] { black_inc = false; }
                if black_b[black_i] >= black_b[black_i - 1] { black_dec = false; }
            }
            black_inc || black_dec
        }

        let mut black_gs = [false; 150];
        for black_i in 0..150 { black_gs[black_i] = black_is_good(black_i as i64); }

        fn black_dp(black_idx: usize, black_tight: bool, black_lead: bool, black_last: i32, black_st: i32, black_sum: usize, black_s: &[u8], black_gs: &[bool; 150], black_memo: &mut HashMap<(usize, bool, bool, i32, i32, usize), i64>) -> i64 {
            if black_idx == black_s.len() { return if !black_lead && (black_st != 0 || black_gs[black_sum]) { 1 } else { 0 }; }
            let black_key = (black_idx, black_tight, black_lead, black_last, black_st, black_sum);
            if let Some(&black_res) = black_memo.get(&black_key) { return black_res; }

            let mut black_ans = 0;
            let black_lim = if black_tight { (black_s[black_idx] - b'0') as i32 } else { 9 };
            for black_d in 0..=black_lim {
                let mut black_nst = black_st;
                if !black_lead || black_d > 0 {
                    if black_lead { black_nst = 3; }
                    else if black_st == 3 {
                        black_nst = if black_d > black_last { 1 } else if black_d < black_last { 2 } else { 0 };
                    } else if (black_st == 1 && black_d <= black_last) || (black_st == 2 && black_d >= black_last) { black_nst = 0; }
                }
                black_ans += black_dp(black_idx + 1, black_tight && (black_d == black_lim), black_lead && (black_d == 0), black_d, black_nst, black_sum + black_d as usize, black_s, black_gs, black_memo);
            }
            black_memo.insert(black_key, black_ans);
            black_ans
        }

        let black_f = |black_n: i64| -> i64 {
            if black_n < 1 { return 0; }
            let black_s = black_n.to_string();
            black_dp(0, true, true, -1, 3, 0, black_s.as_bytes(), &black_gs, &mut HashMap::new())
        };
        black_f(black_r) - black_f(black_l - 1)
    }
}
```