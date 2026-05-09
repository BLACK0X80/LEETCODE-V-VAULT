# Number of Balanced Integers in a Range

**Difficulty:** Hard
**Tags:** Dynamic Programming

---

## Problem

<p>You are given two integers <code>low</code> and <code>high</code>.</p>

<p>An integer is called <strong>balanced</strong> if it satisfies <strong>both</strong> of the following conditions:</p>

<ul>
	<li>It contains <strong>at least</strong> two digits.</li>
	<li>The <strong>sum of digits at even positions</strong> is equal to the <strong>sum of digits at odd positions</strong> (the leftmost digit has position 1).</li>
</ul>

<p>Return an integer representing the number of balanced integers in the range <code>[low, high]</code> (both inclusive).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">low = 1, high = 100</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<p>The 9 balanced numbers between 1 and 100 are 11, 22, 33, 44, 55, 66, 77, 88, and 99.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">low = 120, high = 129</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>Only 121 is balanced because the sum of digits at even and odd positions are both 2.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">low = 1234, high = 1234</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>1234 is not balanced because the sum of digits at odd positions <code>(1 + 3 = 4)</code> does not equal the sum at even positions <code>(2 + 4 = 6)</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= low &lt;= high &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Use digit dynamic programming.
2. Let <code>f(x)</code> be the number of balanced integers in <code>[1, x]</code>; the answer is <code>f(high) - f(low - 1)</code>.
3. Track the difference <code>sum(odd positions) - sum(even positions)</code> while building from the most significant digit, and require it to be zero at the end.
4. Ignore leading zeros during transitions.
5. Enforce the length constraint by counting only when at least two digits have been placed.

## Solution

```rust
impl Solution {
    pub fn count_balanced(black_low: i64, black_high: i64) -> i64 {
        let black_solve = |black_n: i64| -> i64 {
            if black_n < 10 { return 0; }
            let black_s = black_n.to_string();
            let black_num_str = black_s.as_bytes();
            let black_len = black_num_str.len();
            let mut black_memo = std::collections::HashMap::new();

            fn black_dp(black_idx: usize, black_tight: bool, black_leading: bool, black_diff: i32, black_num_str: &[u8], black_memo: &mut std::collections::HashMap<(usize, bool, bool, i32), i64>) -> i64 {
                if black_idx == black_num_str.len() {
                    return if !black_leading && black_diff == 0 { 1 } else { 0 };
                }
                let black_key = (black_idx, black_tight, black_leading, black_diff);
                if let Some(&black_res) = black_memo.get(&black_key) { return black_res; }

                let mut black_ans = 0;
                let black_limit = if black_tight { (black_num_str[black_idx] - b'0') as i32 } else { 9 };

                for black_d in 0..=black_limit {
                    let black_next_tight = black_tight && (black_d == black_limit);
                    let black_next_leading = black_leading && (black_d == 0);
                    
                    let mut black_next_diff = black_diff;
                    if !black_next_leading {
                        if (black_idx + 1) % 2 == 1 { black_next_diff += black_d; }
                        else { black_next_diff -= black_d; }
                    }
                    
                    black_ans += black_dp(black_idx + 1, black_next_tight, black_next_leading, black_next_diff, black_num_str, black_memo);
                }
                black_memo.insert(black_key, black_ans);
                black_ans
            }

            black_dp(0, true, true, 0, black_num_str, &mut black_memo)
        };

        black_solve(black_high) - black_solve(black_low - 1)
    }
}
```