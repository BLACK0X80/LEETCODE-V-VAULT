# Total Waviness of Numbers in Range II

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>You are given two integers <code>num1</code> and <code>num2</code> representing an <strong>inclusive</strong> range <code>[num1, num2]</code>.</p>

<p>The <strong>waviness</strong> of a number is defined as the total count of its <strong>peaks</strong> and <strong>valleys</strong>:</p>

<ul>
	<li>A digit is a <strong>peak</strong> if it is <strong>strictly greater</strong> than both of its immediate neighbors.</li>
	<li>A digit is a <strong>valley</strong> if it is <strong>strictly less</strong> than both of its immediate neighbors.</li>
	<li>The first and last digits of a number <strong>cannot</strong> be peaks or valleys.</li>
	<li>Any number with fewer than 3 digits has a waviness of 0.</li>
</ul>
Return the total sum of waviness for all numbers in the range <code>[num1, num2]</code>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num1 = 120, num2 = 130</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>In the range <code>[120, 130]</code>:</p>

<ul>
	<li><code>120</code>: middle digit 2 is a peak, waviness = 1.</li>
	<li><code>121</code>: middle digit 2 is a peak, waviness = 1.</li>
	<li><code>130</code>: middle digit 3 is a peak, waviness = 1.</li>
	<li>All other numbers in the range have a waviness of 0.</li>
</ul>

<p>Thus, total waviness is <code>1 + 1 + 1 = 3</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num1 = 198, num2 = 202</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>In the range <code>[198, 202]</code>:</p>

<ul>
	<li><code>198</code>: middle digit 9 is a peak, waviness = 1.</li>
	<li><code>201</code>: middle digit 0 is a valley, waviness = 1.</li>
	<li><code>202</code>: middle digit 0 is a valley, waviness = 1.</li>
	<li>All other numbers in the range have a waviness of 0.</li>
</ul>

<p>Thus, total waviness is <code>1 + 1 + 1 = 3</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num1 = 4848, num2 = 4848</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Number <code>4848</code>: the second digit 8 is a peak, and the third digit 4 is a valley, giving a waviness of 2.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num1 &lt;= num2 &lt;= 10<sup>15</sup></code>​​​​​​​</li>
</ul>


## Hints

1. Use digit dynamic programming
2. Build a digit-DP state <code>(position, tight, lastDigit, secondLastDigit)</code>

## Solution

```rust
impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        fn black_solve(black_n: i64) -> i64 {
            if black_n < 100 { return 0; }
            let black_s = black_n.to_string().bytes().map(|b| (b - b'0') as i8).collect::<Vec<_>>();
            let mut black_dp = std::collections::HashMap::new();
            fn black_dfs(black_i: usize, black_t: bool, black_std: bool, black_p2: i8, black_p1: i8, black_s: &Vec<i8>, black_memo: &mut std::collections::HashMap<(usize, bool, bool, i8, i8), (i64, i64)>) -> (i64, i64) {
                if black_i == black_s.len() { return (if black_std { 1 } else { 0 }, 0); }
                if let Some(&black_res) = black_memo.get(&(black_i, black_t, black_std, black_p2, black_p1)) { return black_res; }
                let (mut black_c_sum, mut black_w_sum, black_lim) = (0, 0, if black_t { black_s[black_i] } else { 9 });
                for black_d in 0..=black_lim {
                    let black_nt = black_t && (black_d == black_lim);
                    let (black_c, black_w) = if !black_std { if black_d == 0 { black_dfs(black_i + 1, black_nt, false, 10, 10, black_s, black_memo) } else { black_dfs(black_i + 1, black_nt, true, 10, black_d, black_s, black_memo) } }
                    else { let black_wav = if black_p1 != 10 && black_p2 != 10 && ((black_p2 < black_p1 && black_p1 > black_d) || (black_p2 > black_p1 && black_p1 < black_d)) { 1 } else { 0 };
                        let black_nxt = black_dfs(black_i + 1, black_nt, true, black_p1, black_d, black_s, black_memo); (black_nxt.0, black_nxt.1 + black_nxt.0 * black_wav) };
                    black_c_sum += black_c; black_w_sum += black_w;
                }
                black_memo.insert((black_i, black_t, black_std, black_p2, black_p1), (black_c_sum, black_w_sum)); (black_c_sum, black_w_sum)
            }
            black_dfs(0, true, false, 10, 10, &black_s, &mut black_dp).1
        }
        black_solve(num2) - black_solve(num1 - 1)
    }
}
```