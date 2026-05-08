# Count No-Zero Pairs That Sum to N

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>A <strong>no-zero</strong> integer is a <strong>positive</strong> integer that <strong>does not contain the digit</strong> 0 in its decimal representation.</p>

<p>Given an integer <code>n</code>, count the number of pairs <code>(a, b)</code> where:</p>

<ul>
	<li><code>a</code> and <code>b</code> are <strong>no-zero</strong> integers.</li>
	<li><code>a + b = n</code></li>
</ul>

<p>Return an integer denoting the number of such pairs.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only pair is <code>(1, 1)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The pairs are <code>(1, 2)</code> and <code>(2, 1)</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 11</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<p>The pairs are <code>(2, 9)</code>, <code>(3, 8)</code>, <code>(4, 7)</code>, <code>(5, 6)</code>, <code>(6, 5)</code>, <code>(7, 4)</code>, <code>(8, 3)</code>, and <code>(9, 2)</code>. Note that <code>(1, 10)</code> and <code>(10, 1)</code> do not satisfy the conditions because 10 contains 0 in its decimal representation.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Use digit DP over the decimal representation of <code>n</code>.
2. At each digit, track whether a carry is present and whether <code>a</code> or <code>b</code> has used a zero so far.
3. Transition by choosing digits for <code>a</code> and <code>b</code> that add up (with carry) to the current digit of <code>n</code>.
4. Subtract the cases where either number ends up being zero-containing when <code>n</code> itself is no-zero.

## Solution

```rust
impl Solution {
    pub fn count_no_zero_pairs(black_n: i64) -> i64 {
        let mut black_a = Vec::new();
        let mut black_temp_n = black_n;
        while black_temp_n > 0 {
            black_a.push((black_temp_n % 10) as i32);
            black_temp_n /= 10;
        }
        let black_sz = black_a.len();
        let mut black_dp = vec![vec![vec![vec![vec![-1; 2]; 2]; 2]; 2]; 17];

        Self::black_solve(0, 0, 0, 0, 1, black_sz, &black_a, &mut black_dp)
    }

    fn black_solve(
        black_i: usize,
        black_car: usize,
        black_done1: usize,
        black_done2: usize,
        black_started: usize,
        black_sz: usize,
        black_a: &Vec<i32>,
        black_dp: &mut Vec<Vec<Vec<Vec<Vec<i64>>>>>,
    ) -> i64 {
        if black_i == black_sz {
            return if black_car == 0 && black_started != 0 { 1 } else { 0 };
        }

        if black_dp[black_i][black_car][black_done1][black_done2][black_started] != -1 {
            return black_dp[black_i][black_car][black_done1][black_done2][black_started];
        }

        if black_done1 != 0 && black_done2 != 0 {
            return 0;
        }

        let mut black_ans = 0;

        if black_a[black_i] == black_car as i32 {
            black_ans += Self::black_solve(black_i + 1, 0, 1, 1, black_started, black_sz, black_a, black_dp);
        }

        if black_done1 != 0 {
            for black_jj in 1..=9 {
                let mut black_curr = black_jj + black_car;
                let black_carry_next = black_curr / 10;
                black_curr %= 10;
                if black_curr == black_a[black_i] as usize {
                    black_ans += Self::black_solve(black_i + 1, black_carry_next, 1, black_done2, 1, black_sz, black_a, black_dp);
                }
            }
        }

        if black_done2 != 0 {
            for black_ii in 1..=9 {
                let mut black_curr = black_ii + black_car;
                let black_carry_next = black_curr / 10;
                black_curr %= 10;
                if black_curr == black_a[black_i] as usize {
                    black_ans += Self::black_solve(black_i + 1, black_carry_next, black_done1, 1, 1, black_sz, black_a, black_dp);
                }
            }
        }

        if black_done1 == 0 && black_done2 == 0 {
            for black_ii in 1..=9 {
                for black_jj in 1..=9 {
                    let mut black_curr = black_ii + black_jj + black_car;
                    let black_carry_next = black_curr / 10;
                    black_curr %= 10;
                    if black_curr == black_a[black_i] as usize {
                        black_ans += Self::black_solve(black_i + 1, black_carry_next, 1, black_done2, 0, black_sz, black_a, black_dp);
                        black_ans += Self::black_solve(black_i + 1, black_carry_next, black_done1, 1, 0, black_sz, black_a, black_dp);
                        black_ans += Self::black_solve(black_i + 1, black_carry_next, black_done1, black_done2, black_started, black_sz, black_a, black_dp);
                    }
                }
            }
        }

        black_dp[black_i][black_car][black_done1][black_done2][black_started] = black_ans;
        black_ans
    }
}
```