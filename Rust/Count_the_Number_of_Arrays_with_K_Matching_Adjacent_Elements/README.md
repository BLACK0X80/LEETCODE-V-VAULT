# Count the Number of Arrays with K Matching Adjacent Elements

**Difficulty:** Hard
**Tags:** Math, Combinatorics

---

## Problem

<p>You are given three integers <code>n</code>, <code>m</code>, <code>k</code>. A <strong>good array</strong> <code>arr</code> of size <code>n</code> is defined as follows:</p>

<ul>
	<li>Each element in <code>arr</code> is in the <strong>inclusive</strong> range <code>[1, m]</code>.</li>
	<li><em>Exactly</em> <code>k</code> indices <code>i</code> (where <code>1 &lt;= i &lt; n</code>) satisfy the condition <code>arr[i - 1] == arr[i]</code>.</li>
</ul>

<p>Return the number of <strong>good arrays</strong> that can be formed.</p>

<p>Since the answer may be very large, return it <strong>modulo </strong><code>10<sup>9 </sup>+ 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, m = 2, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There are 4 good arrays. They are <code>[1, 1, 2]</code>, <code>[1, 2, 2]</code>, <code>[2, 1, 1]</code> and <code>[2, 2, 1]</code>.</li>
	<li>Hence, the answer is 4.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, m = 2, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The good arrays are <code>[1, 1, 1, 2]</code>, <code>[1, 1, 2, 2]</code>, <code>[1, 2, 2, 2]</code>, <code>[2, 1, 1, 1]</code>, <code>[2, 2, 1, 1]</code> and <code>[2, 2, 2, 1]</code>.</li>
	<li>Hence, the answer is 6.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, m = 2, k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The good arrays are <code>[1, 2, 1, 2, 1]</code> and <code>[2, 1, 2, 1, 2]</code>. Hence, the answer is 2.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= m &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= k &lt;= n - 1</code></li>
</ul>


## Hints

1. The first position <code>arr[0]</code> has <code>m</code> choices.
2. For each of the remaining <code>n - 1</code> indices, <code>0 < i < n</code>, select <code>k</code> positions from left to right and set <code>arr[i] = arr[i - 1]</code>.
3. For all other indices, <code>set arr[i] != arr[i - 1]</code> with (<code>m - 1</code>) choices for each of the <code>n - 1 - k</code> positions.

## Solution

```rust
impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = n as usize;
        let black_m = m as i64;
        let black_k = k as usize;

        let mut black_fact = vec![1i64; black_n + 1];
        let mut black_inv = vec![1i64; black_n + 1];
        for i in 1..=black_n { black_fact[i] = (black_fact[i - 1] * i as i64) % black_mod; }
        
        fn power(mut a: i64, mut b: i64, m: i64) -> i64 {
            let mut res = 1;
            while b > 0 {
                if b % 2 == 1 { res = (res * a) % m; }
                a = (a * a) % m;
                b /= 2;
            }
            res
        }
        black_inv[black_n] = power(black_fact[black_n], black_mod - 2, black_mod);
        for i in (0..black_n).rev() { black_inv[i] = (black_inv[i + 1] * (i + 1) as i64) % black_mod; }

        let ncr = |n: usize, r: usize| -> i64 {
            if r > n { return 0; }
            black_fact[n] * black_inv[r] % black_mod * black_inv[n - r] % black_mod
        };

        
        let mut black_res = black_m * ncr(black_n - 1, black_k) % black_mod;
        black_res = (black_res * power(black_m - 1, (black_n - 1 - black_k) as i64, black_mod)) % black_mod;

        black_res as i32
    }
}
```