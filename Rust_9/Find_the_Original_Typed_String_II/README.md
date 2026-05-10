# Find the Original Typed String II

**Difficulty:** Hard
**Tags:** String, Dynamic Programming, Prefix Sum

---

## Problem

<p>Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and <strong>may</strong> press a key for too long, resulting in a character being typed <strong>multiple</strong> times.</p>

<p>You are given a string <code>word</code>, which represents the <strong>final</strong> output displayed on Alice&#39;s screen. You are also given a <strong>positive</strong> integer <code>k</code>.</p>

<p>Return the total number of <em>possible</em> original strings that Alice <em>might</em> have intended to type, if she was trying to type a string of size <strong>at least</strong> <code>k</code>.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;aabbccdd&quot;, k = 7</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible strings are: <code>&quot;aabbccdd&quot;</code>, <code>&quot;aabbccd&quot;</code>, <code>&quot;aabbcdd&quot;</code>, <code>&quot;aabccdd&quot;</code>, and <code>&quot;abbccdd&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;aabbccdd&quot;, k = 8</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only possible string is <code>&quot;aabbccdd&quot;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;aaabbb&quot;, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>word</code> consists only of lowercase English letters.</li>
	<li><code>1 &lt;= k &lt;= 2000</code></li>
</ul>


## Hints

1. Instead of solving for at least <code>k</code>, can we solve for at most <code>k - 1</code> length?

## Solution

```rust
impl Solution {
    pub fn possible_string_count(black_word: String, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_chars: Vec<char> = black_word.chars().collect();
        let black_n = black_chars.len();
        let black_k = black_k as usize;

        let mut black_groups = Vec::new();
        let mut black_i = 0;
        while black_i < black_n {
            let mut black_j = black_i;
            while black_j < black_n && black_chars[black_j] == black_chars[black_i] {
                black_j += 1;
            }
            black_groups.push((black_j - black_i) as i64);
            black_i = black_j;
        }

        let black_m = black_groups.len();
        if black_m >= black_k {
            let mut black_total_ways = 1i64;
            for black_g in black_groups {
                black_total_ways = (black_total_ways * black_g) % black_mod;
            }
            return black_total_ways as i32;
        }

        let mut black_dp = vec![0i64; black_k];
        black_dp[0] = 1;

        for black_g in black_groups.iter().map(|&x| x as usize) {
            let mut black_next_dp = vec![0i64; black_k];
            let mut black_prefix_sum = vec![0i64; black_k + 1];
            for black_idx in 0..black_k {
                black_prefix_sum[black_idx + 1] = (black_prefix_sum[black_idx] + black_dp[black_idx]) % black_mod;
            }

            for black_s in 0..black_k {
                let black_low = black_s.saturating_sub(black_g);
                let black_high = black_s.saturating_sub(1);
                if black_s > 0 {
                    let black_val = (black_prefix_sum[black_high + 1] - black_prefix_sum[black_low] + black_mod) % black_mod;
                    black_next_dp[black_s] = black_val;
                }
            }
            black_dp = black_next_dp;
        }

        let mut black_invalid_ways = 0i64;
        for black_v in black_dp {
            black_invalid_ways = (black_invalid_ways + black_v) % black_mod;
        }

        let mut black_total_all = 1i64;
        for black_g in black_groups {
            black_total_all = (black_total_all * black_g) % black_mod;
        }

        ((black_total_all - black_invalid_ways + black_mod) % black_mod) as i32
    }
}
```