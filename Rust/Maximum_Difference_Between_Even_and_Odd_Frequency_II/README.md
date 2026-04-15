# Maximum Difference Between Even and Odd Frequency II

**Difficulty:** Hard
**Tags:** String, Sliding Window, Enumeration, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> and an integer <code>k</code>. Your task is to find the <strong>maximum</strong> difference between the frequency of <strong>two</strong> characters, <code>freq[a] - freq[b]</code>, in a <span data-keyword="substring">substring</span> <code>subs</code> of <code>s</code>, such that:</p>

<ul>
	<li><code>subs</code> has a size of <strong>at least</strong> <code>k</code>.</li>
	<li>Character <code>a</code> has an <em>odd frequency</em> in <code>subs</code>.</li>
	<li>Character <code>b</code> has a <strong>non-zero</strong> <em>even frequency</em> in <code>subs</code>.</li>
</ul>

<p>Return the <strong>maximum</strong> difference.</p>

<p><strong>Note</strong> that <code>subs</code> can contain more than 2 <strong>distinct</strong> characters.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;12233&quot;, k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>For the substring <code>&quot;12233&quot;</code>, the frequency of <code>&#39;1&#39;</code> is 1 and the frequency of <code>&#39;3&#39;</code> is 2. The difference is <code>1 - 2 = -1</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;1122211&quot;, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>For the substring <code>&quot;11222&quot;</code>, the frequency of <code>&#39;2&#39;</code> is 3 and the frequency of <code>&#39;1&#39;</code> is 2. The difference is <code>3 - 2 = 1</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;110&quot;, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= s.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>s</code> consists only of digits <code>&#39;0&#39;</code> to <code>&#39;4&#39;</code>.</li>
	<li>The input is generated that at least one substring has a character with an even frequency and a character with an odd frequency.</li>
	<li><code>1 &lt;= k &lt;= s.length</code></li>
</ul>


## Hints

1. Fix the two characters.
2. Use prefix sum (maintain 2 characters' parities as status).

## Solution

```rust
use std::cmp::{max, min};

struct BlackMinBIT {
    black_n: usize,
    black_data: Vec<i32>,
}

impl BlackMinBIT {
    const BLACK_MAX: i32 = i32::MAX;

    fn new(black_len: usize) -> Self {
        Self {
            black_n: black_len,
            black_data: vec![Self::BLACK_MAX; black_len + 2],
        }
    }

    fn insert(&mut self, black_idx: usize, black_val: i32) {
        let mut black_i = (black_idx + 1) as i32;
        while black_i <= (self.black_n as i32) {
            self.black_data[black_i as usize] = min(self.black_data[black_i as usize], black_val);
            black_i += black_i & -black_i;
        }
    }

    fn get_min(&self, black_idx: usize) -> i32 {
        let mut black_res = Self::BLACK_MAX;
        let mut black_i = (black_idx + 1) as i32;
        while black_i > 0 {
            black_res = min(black_res, self.black_data[black_i as usize]);
            black_i -= black_i & -black_i;
        }
        black_res
    }
}

impl Solution {
    pub fn max_difference(black_s: String, black_k: i32) -> i32 {
        let black_length = black_s.len();
        let black_bytes = black_s.as_bytes();
        let black_k = black_k as usize;
        let mut black_final_res = i32::MIN;

        for black_first in 0..5 {
            for black_second in 0..5 {
                if black_first == black_second { continue; }

                let mut black_diff = vec![0i32; black_length + 1];
                let mut black_parity_a = vec![0usize; black_length + 1];
                let mut black_parity_b = vec![0usize; black_length + 1];
                let mut black_count_b = vec![0usize; black_length + 1];

                let black_f_byte = black_first + b'0';
                let black_s_byte = black_second + b'0';

                for black_i in 1..=black_length {
                    let black_digit = black_bytes[black_i - 1];
                    black_diff[black_i] = black_diff[black_i - 1] + 
                        if black_digit == black_f_byte { 1 } else if black_digit == black_s_byte { -1 } else { 0 };
                    black_parity_a[black_i] = (black_parity_a[black_i - 1] + if black_digit == black_f_byte { 1 } else { 0 }) & 1;
                    black_parity_b[black_i] = (black_parity_b[black_i - 1] + if black_digit == black_s_byte { 1 } else { 0 }) & 1;
                    black_count_b[black_i] = black_count_b[black_i - 1] + if black_digit == black_s_byte { 1 } else { 0 };
                }

                let mut black_storage = vec![
                    vec![BlackMinBIT::new(black_length + 1), BlackMinBIT::new(black_length + 1)],
                    vec![BlackMinBIT::new(black_length + 1), BlackMinBIT::new(black_length + 1)],
                ];

                for black_j in 0..=black_length {
                    if black_j >= black_k {
                        let black_back = black_j - black_k;
                        let black_p_a = black_parity_a[black_back];
                        let black_p_b = black_parity_b[black_back];
                        let black_b_cnt = black_count_b[black_back];
                        black_storage[black_p_a][black_p_b].insert(black_b_cnt, black_diff[black_back]);
                    }

                    if black_j > 0 && black_count_b[black_j] > 0 {
                        let black_alt_a = 1 - black_parity_a[black_j];
                        let black_cur_b = black_parity_b[black_j];
                        let black_min_prev = black_storage[black_alt_a][black_cur_b].get_min(black_count_b[black_j] - 1);

                        if black_min_prev != BlackMinBIT::BLACK_MAX {
                            black_final_res = max(black_final_res, black_diff[black_j] - black_min_prev);
                        }
                    }
                }
            }
        }

        if black_final_res == i32::MIN { 0 } else { black_final_res }
    }
}
```