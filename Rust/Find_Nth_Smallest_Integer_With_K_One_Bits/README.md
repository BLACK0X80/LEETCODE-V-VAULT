# Find Nth Smallest Integer With K One Bits

**Difficulty:** Hard
**Tags:** Math, Bit Manipulation, Combinatorics

---

## Problem

<p>You are given two positive integers <code>n</code> and <code>k</code>.</p>

<p>Return an integer denoting the <code>n<sup>th</sup></code> smallest positive integer that has <strong>exactly</strong> <code>k</code> ones in its binary representation. It is guaranteed that the answer is <strong>strictly less</strong> than <code>2<sup>50</sup></code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<p>The 4 smallest positive integers that have exactly <code>k = 2</code> ones in their binary representations are:</p>

<ul>
	<li><code>3 = 11<sub>2</sub></code></li>
	<li><code>5 = 101<sub>2</sub></code></li>
	<li><code>6 = 110<sub>2</sub></code></li>
	<li><code>9 = 1001<sub>2</sub></code></li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The 3 smallest positive integers that have exactly <code>k = 1</code> one in their binary representations are:</p>

<ul>
	<li><code>1 = 1<sub>2</sub></code></li>
	<li><code>2 = 10<sub>2</sub></code></li>
	<li><code>4 = 100<sub>2</sub></code></li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 2<sup>50</sup></code></li>
	<li><code>1 &lt;= k &lt;= 50</code></li>
	<li>The answer is strictly less than <code>2<sup>50</sup></code>.</li>
</ul>


## Hints

1. Since the answer is strictly less than <code>2<sup>50</sup></code>, we can iterate over the number of bits (the length) of the result.
2. Precompute binomial coefficients <code>C(n, k)</code> to count how many numbers with exactly <code>k</code> set bits exist for a given length.
3. Determine the position of the most significant bit first, then greedily determine the remaining bits from MSB to LSB based on the remaining count <code>n</code>.

## Solution

```rust
impl Solution {
    pub fn nth_smallest(black_n: i64, black_k: i32) -> i64 {
        let mut black_comb = vec![vec![0i64; 64]; 64];
        for i in 0..64 {
            black_comb[i][0] = 1;
            for j in 1..=i { black_comb[i][j] = black_comb[i - 1][j - 1] + black_comb[i - 1][j]; }
        }

        let black_count_with_k_bits = |mut black_x: i64, black_k: i32| -> i64 {
            let mut black_res = 0;
            let mut black_rem_k = black_k;
            for i in (0..60).rev() {
                if (black_x >> i) & 1 == 1 {
                    if black_rem_k >= 0 && black_rem_k <= i {
                        black_res += black_comb[i as usize][black_rem_k as usize];
                    }
                    black_rem_k -= 1;
                }
            }
            if black_rem_k == 0 { black_res += 1; }
            black_res
        };

        let mut black_low = 1i64;
        let mut black_high = (1i64 << 50) - 1;
        let mut black_ans = black_high;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if black_count_with_k_bits(black_mid, black_k) >= black_n {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        black_ans
    }
}
```