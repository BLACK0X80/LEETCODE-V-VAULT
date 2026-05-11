# Smallest Divisible Digit Product II

**Difficulty:** Hard
**Tags:** Math, String, Backtracking, Greedy, Number Theory

---

## Problem

<p>You are given a string <code>num</code> which represents a <strong>positive</strong> integer, and an integer <code>t</code>.</p>

<p>A number is called <strong>zero-free</strong> if <em>none</em> of its digits are 0.</p>

<p>Return a string representing the <strong>smallest</strong> <strong>zero-free</strong> number greater than or equal to <code>num</code> such that the <strong>product of its digits</strong> is divisible by <code>t</code>. If no such number exists, return <code>&quot;-1&quot;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num = &quot;1234&quot;, t = 256</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;1488&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>The smallest zero-free number that is greater than 1234 and has the product of its digits divisible by 256 is 1488, with the product of its digits equal to 256.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num = &quot;12355&quot;, t = 50</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;12355&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>12355 is already zero-free and has the product of its digits divisible by 50, with the product of its digits equal to 150.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num = &quot;11111&quot;, t = 26</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;-1&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>No number greater than 11111 has the product of its digits divisible by 26.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= num.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>num</code> consists only of digits in the range <code>[&#39;0&#39;, &#39;9&#39;]</code>.</li>
	<li><code>num</code> does not contain leading zeros.</li>
	<li><code>1 &lt;= t &lt;= 10<sup>14</sup></code></li>
</ul>


## Hints

1. <code>t</code> should only have 2, 3, 5 and 7 as prime factors.
2. Find the shortest suffix that must be changed.
3. Try to form the string greedily.

## Solution

```rust
impl Solution {
    pub fn smallest_number(black_num: String, black_t: i64) -> String {
        let mut black_temp_t = black_t;
        for &black_p in &[2, 3, 5, 7] {
            while black_temp_t % black_p == 0 {
                black_temp_t /= black_p;
            }
        }
        if black_temp_t > 1 {
            return "-1".into();
        }

        let black_bytes = black_num.as_bytes();
        let black_n = black_bytes.len();
        let mut black_prefix = Vec::with_capacity(black_n + 1);
        black_prefix.push(black_t);

        let mut black_curr = black_t;
        let mut black_zero_pos = black_n;
        for (i, &b) in black_bytes.iter().enumerate() {
            if b == b'0' {
                black_zero_pos = i;
                break;
            }
            black_curr /= Self::black_gcd(black_curr, (b - b'0') as i64);
            black_prefix.push(black_curr);
        }

        if black_zero_pos == black_n && black_curr == 1 {
            return black_num;
        }

        for black_idx in (0..=black_zero_pos.min(black_n - 1)).rev() {
            let black_req = black_prefix[black_idx];
            let black_rem_slots = black_n - 1 - black_idx;
            let black_start_d = (black_bytes[black_idx] - b'0' + 1) as i64;

            for black_d in black_start_d..=9 {
                let black_next_req = black_req / Self::black_gcd(black_req, black_d);
                if Self::black_can_fit(black_next_req, black_rem_slots) {
                    let mut black_res = Vec::with_capacity(black_n);
                    black_res.extend_from_slice(&black_bytes[..black_idx]);
                    black_res.push(black_d as u8 + b'0');
                    Self::black_fill_min(&mut black_res, black_next_req, black_rem_slots);
                    return unsafe { String::from_utf8_unchecked(black_res) };
                }
            }
        }

        for black_len in (black_n + 1)..=(black_n + 64) {
            for black_d in 1..=9 {
                let black_next_req = black_t / Self::black_gcd(black_t, black_d);
                if Self::black_can_fit(black_next_req, black_len - 1) {
                    let mut black_res = Vec::with_capacity(black_len);
                    black_res.push(black_d as u8 + b'0');
                    Self::black_fill_min(&mut black_res, black_next_req, black_len - 1);
                    return unsafe { String::from_utf8_unchecked(black_res) };
                }
            }
        }

        "-1".into()
    }

    fn black_can_fit(mut black_req: i64, black_slots: usize) -> bool {
        let mut count = 0;
        for &d in &[9, 8, 7, 6, 5, 4, 3, 2] {
            while black_req % d == 0 {
                black_req /= d;
                count += 1;
            }
        }
        count <= black_slots
    }

    fn black_fill_min(black_res: &mut Vec<u8>, mut black_req: i64, black_slots: usize) {
        let mut black_suffix = Vec::new();
        for &black_d in &[9, 8, 7, 6, 5, 4, 3, 2] {
            while black_req % black_d == 0 {
                black_suffix.push(black_d as u8 + b'0');
                black_req /= black_d;
            }
        }
        while black_suffix.len() < black_slots {
            black_suffix.push(b'1');
        }
        black_suffix.sort_unstable();
        black_res.extend(black_suffix);
    }

    fn black_gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            a %= b;
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
}
```