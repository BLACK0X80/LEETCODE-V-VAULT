# Nth Magical Number

**Difficulty:** Hard
**Tags:** Math, Binary Search

---

## Problem

<p>A positive integer is <em>magical</em> if it is divisible by either <code>a</code> or <code>b</code>.</p>

<p>Given the three integers <code>n</code>, <code>a</code>, and <code>b</code>, return the <code>n<sup>th</sup></code> magical number. Since the answer may be very large, <strong>return it modulo </strong><code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 1, a = 2, b = 3
<strong>Output:</strong> 2
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 4, a = 2, b = 3
<strong>Output:</strong> 6
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
	<li><code>2 &lt;= a, b &lt;= 4 * 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn nth_magical_number(black_n: i32, black_a: i32, black_b: i32) -> i32 {
        let (black_a, black_b) = (black_a as i64, black_b as i64);
        let black_mod = 1_000_000_007i64;
        
        fn black_gcd(mut black_x: i64, mut black_y: i64) -> i64 {
            while black_y != 0 { black_x %= black_y; std::mem::swap(&mut black_x, &mut black_y); }
            black_x
        }

        let black_lcm = (black_a * black_b) / black_gcd(black_a, black_b);
        let mut black_low = 2i64;
        let mut black_high = 10i64.pow(15);
        let mut black_ans = 0i64;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            let bravexuneth = black_mid / black_a + black_mid / black_b - black_mid / black_lcm;
            if bravexuneth >= black_n as i64 {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        (black_ans % black_mod) as i32
    }
}
```