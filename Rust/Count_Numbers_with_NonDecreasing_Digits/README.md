# Count Numbers with Non-Decreasing Digits 

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming

---

## Problem

<p>You are given two integers, <code>l</code> and <code>r</code>, represented as strings, and an integer <code>b</code>. Return the count of integers in the inclusive range <code>[l, r]</code> whose digits are in <strong>non-decreasing</strong> order when represented in base <code>b</code>.</p>

<p>An integer is considered to have <strong>non-decreasing</strong> digits if, when read from left to right (from the most significant digit to the least significant digit), each digit is greater than or equal to the previous one.</p>

<p>Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = &quot;23&quot;, r = &quot;28&quot;, b = 8</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The numbers from 23 to 28 in base 8 are: 27, 30, 31, 32, 33, and 34.</li>
	<li>Out of these, 27, 33, and 34 have non-decreasing digits. Hence, the output is 3.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = &quot;2&quot;, r = &quot;7&quot;, b = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The numbers from 2 to 7 in base 2 are: 10, 11, 100, 101, 110, and 111.</li>
	<li>Out of these, 11 and 111 have non-decreasing digits. Hence, the output is 2.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code><font face="monospace">1 &lt;= l.length &lt;= r.length &lt;= 100</font></code></li>
	<li><code>2 &lt;= b &lt;= 10</code></li>
	<li><code>l</code> and <code>r</code> consist only of digits.</li>
	<li>The value represented by <code>l</code> is less than or equal to the value represented by <code>r</code>.</li>
	<li><code>l</code> and <code>r</code> do not contain leading zeros.</li>
</ul>


## Hints

1. Use digit dynamic programming.

## Solution

```rust
impl Solution {
    pub fn count_numbers(black_l: String, black_r: String, black_b: i32) -> i32 {
        let black_mod = 1_000_000_007;

        fn black_to_base(black_num_str: String, black_b: i32) -> Vec<i32> {
            let mut black_digits = Vec::new();
            if black_num_str == "0" { return vec![0]; }
            let mut black_bytes: Vec<u8> = black_num_str.bytes().map(|b| b - b'0').collect();
            while !black_bytes.is_empty() {
                let mut black_rem = 0u64;
                let mut black_next = Vec::with_capacity(black_bytes.len());
                for &b_val in &black_bytes {
                    let black_curr = black_rem * 10 + b_val as u64;
                    let black_q = black_curr / black_b as u64;
                    black_rem = black_curr % black_b as u64;
                    if !black_next.is_empty() || black_q > 0 {
                        black_next.push(black_q as u8);
                    }
                }
                black_digits.push(black_rem as i32);
                black_bytes = black_next;
            }
            black_digits.reverse();
            black_digits
        }

        fn black_solve(black_num: &[i32], black_b: i32, black_mod: i32) -> i32 {
            let black_n = black_num.len();
            let mut black_dp = vec![vec![vec![vec![-1; 2]; 2]; black_b as usize + 1]; black_n];
            
            fn black_memo(
                black_idx: usize, black_tight: bool, black_last: i32, black_started: bool,
                black_num: &[i32], black_b: i32, black_mod: i32,
                black_dp: &mut Vec<Vec<Vec<Vec<i32>>>>
            ) -> i32 {
                if black_idx == black_num.len() { return 1; }
                let (bt, bs) = (black_tight as usize, black_started as usize);
                if black_dp[black_idx][black_last as usize][bt][bs] != -1 {
                    return black_dp[black_idx][black_last as usize][bt][bs];
                }

                let black_limit = if black_tight { black_num[black_idx] } else { black_b - 1 };
                let mut black_ans = 0;
                for black_d in 0..=black_limit {
                    if black_started && black_d < black_last { continue; }
                    let black_new_started = black_started || (black_d != 0);
                    let black_new_tight = black_tight && (black_d == black_limit);
                    black_ans = (black_ans + black_memo(
                        black_idx + 1, black_new_tight, black_d, black_new_started,
                        black_num, black_b, black_mod, black_dp
                    )) % black_mod;
                }
                black_dp[black_idx][black_last as usize][bt][bs] = black_ans;
                black_ans
            }
            black_memo(0, true, 0, false, black_num, black_b, black_mod, &mut black_dp)
        }

        fn black_sub_one(black_s: String) -> String {
            let mut black_c: Vec<char> = black_s.chars().collect();
            let mut black_i = black_c.len() - 1;
            loop {
                if black_c[black_i] > '0' {
                    black_c[black_i] = (black_c[black_i] as u8 - 1) as char;
                    break;
                } else {
                    black_c[black_i] = '9';
                    if black_i == 0 { break; }
                    black_i -= 1;
                }
            }
            let black_res: String = black_c.into_iter().collect();
            let black_final = black_res.trim_start_matches('0').to_string();
            if black_final.is_empty() { "0".to_string() } else { black_final }
        }

        let black_r_vec = black_to_base(black_r, black_b);
        let black_ans_r = black_solve(&black_r_vec, black_b, black_mod);

        let black_l_minus = black_sub_one(black_l);
        let black_l_vec = black_to_base(black_l_minus, black_b);
        let black_ans_l = black_solve(&black_l_vec, black_b, black_mod);

        (black_ans_r - black_ans_l + black_mod) % black_mod
    }
}
```