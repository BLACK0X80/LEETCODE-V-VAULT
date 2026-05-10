# Count Stepping Numbers in Range

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given two positive integers <code>low</code> and <code>high</code> represented as strings, find the count of <strong>stepping numbers</strong> in the inclusive range <code>[low, high]</code>.</p>

<p>A <strong>stepping number</strong> is an integer such that all of its adjacent digits have an absolute difference of <strong>exactly</strong> <code>1</code>.</p>

<p>Return <em>an integer denoting the count of stepping numbers in the inclusive range</em> <code>[low, high]</code><em>. </em></p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p><strong>Note:</strong> A stepping number should not have a leading zero.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> low = &quot;1&quot;, high = &quot;11&quot;
<strong>Output:</strong> 10
<strong>Explanation: </strong>The stepping numbers in the range [1,11] are 1, 2, 3, 4, 5, 6, 7, 8, 9 and 10. There are a total of 10 stepping numbers in the range. Hence, the output is 10.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> low = &quot;90&quot;, high = &quot;101&quot;
<strong>Output:</strong> 2
<strong>Explanation: </strong>The stepping numbers in the range [90,101] are 98 and 101. There are a total of 2 stepping numbers in the range. Hence, the output is 2. </pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= int(low) &lt;= int(high) &lt; 10<sup>100</sup></code></li>
	<li><code>1 &lt;= low.length, high.length &lt;= 100</code></li>
	<li><code>low</code> and <code>high</code> consist of only digits.</li>
	<li><code>low</code> and <code>high</code> don&#39;t have any leading zeros.</li>
</ul>


## Hints

1. Calculate the number of stepping numbers in the range [1, high] and subtract the number of stepping numbers in the range [1, low - 1].
2. The main problem is calculating the number of stepping numbers in the range [1, x].
3. First, calculate the number of stepping numbers shorter than x in length, which can be done using dynamic programming. (dp[i][j] is the number of i-digit stepping numbers ending with digit j).
4. Finally, calculate the number of stepping numbers that have the same length as x similarly. However, this time we need to maintain whether the prefix (in string) is smaller than or equal to x in the DP state.

## Solution

```rust
impl Solution {
    pub fn count_stepping_numbers(black_low: String, black_high: String) -> i32 {
        let black_mod = 1_000_000_007;
        fn black_calc(black_s: &str, black_m: i32) -> i32 {
            let black_digits: Vec<i32> = black_s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
            let black_n = black_digits.len();
            let mut black_memo = std::collections::HashMap::new();
            fn black_dfs(black_idx: usize, black_prev: i32, black_is_limit: bool, black_is_leading: bool, black_digits: &[i32], black_memo: &mut std::collections::HashMap<(usize, i32, bool, bool), i32>, black_m: i32) -> i32 {
                if black_idx == black_digits.len() { return if black_is_leading { 0 } else { 1 }; }
                let black_state = (black_idx, black_prev, black_is_limit, black_is_leading);
                if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }
                let mut black_ans = 0;
                let black_up = if black_is_limit { black_digits[black_idx] } else { 9 };
                for black_d in 0..=black_up {
                    if black_is_leading {
                        black_ans = (black_ans + black_dfs(black_idx + 1, black_d, black_is_limit && (black_d == black_up), black_is_leading && (black_d == 0), black_digits, black_memo, black_m)) % black_m;
                    } else if (black_d - black_prev).abs() == 1 {
                        black_ans = (black_ans + black_dfs(black_idx + 1, black_d, black_is_limit && (black_d == black_up), false, black_digits, black_memo, black_m)) % black_m;
                    }
                }
                black_memo.insert(black_state, black_ans);
                black_ans
            }
            black_dfs(0, -1, true, true, &black_digits, &mut black_memo, black_m)
        }
        let mut black_res = (black_calc(&black_high, black_mod) - black_calc(&black_low, black_mod) + black_mod) % black_mod;
        let black_low_bytes = black_low.as_bytes();
        let mut black_is_step = true;
        for black_i in 1..black_low_bytes.len() {
            if (black_low_bytes[black_i] as i32 - black_low_bytes[black_i-1] as i32).abs() != 1 { black_is_step = false; break; }
        }
        if black_is_step { black_res = (black_res + 1) % black_mod; }
        black_res
    }
}
```