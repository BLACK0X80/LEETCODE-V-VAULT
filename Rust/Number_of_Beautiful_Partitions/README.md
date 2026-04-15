# Number of Beautiful Partitions

**Difficulty:** Hard
**Tags:** String, Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> that consists of the digits <code>&#39;1&#39;</code> to <code>&#39;9&#39;</code> and two integers <code>k</code> and <code>minLength</code>.</p>

<p>A partition of <code>s</code> is called <strong>beautiful</strong> if:</p>

<ul>
	<li><code>s</code> is partitioned into <code>k</code> non-intersecting substrings.</li>
	<li>Each substring has a length of <strong>at least</strong> <code>minLength</code>.</li>
	<li>Each substring starts with a <strong>prime</strong> digit and ends with a <strong>non-prime</strong> digit. Prime digits are <code>&#39;2&#39;</code>, <code>&#39;3&#39;</code>, <code>&#39;5&#39;</code>, and <code>&#39;7&#39;</code>, and the rest of the digits are non-prime.</li>
</ul>

<p>Return<em> the number of <strong>beautiful</strong> partitions of </em><code>s</code>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters within a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;23542185131&quot;, k = 3, minLength = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> There exists three ways to create a beautiful partition:
&quot;2354 | 218 | 5131&quot;
&quot;2354 | 21851 | 31&quot;
&quot;2354218 | 51 | 31&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;23542185131&quot;, k = 3, minLength = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> There exists one way to create a beautiful partition: &quot;2354 | 218 | 5131&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;3312958&quot;, k = 3, minLength = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> There exists one way to create a beautiful partition: &quot;331 | 29 | 58&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k, minLength &lt;= s.length &lt;= 1000</code></li>
	<li><code>s</code> consists of the digits <code>&#39;1&#39;</code> to <code>&#39;9&#39;</code>.</li>
</ul>


## Hints

1. Try using a greedy approach where you take as many digits as possible from the left of the string for each partition.
2. You can also use a dynamic programming approach, let an array dp where dp[i] is the solution of the problem for the prefix of the string ending at index i, the answer of the problem will be dp[n-1]. What are the transitions of this dp?

## Solution

```rust
impl Solution {
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let black_b = s.as_bytes();
        let black_n = s.len();
        let black_k = k as usize;
        let black_min = min_length as usize;

        let black_is_prime = |black_c: u8| matches!(black_c, b'2' | b'3' | b'5' | b'7');
        if !black_is_prime(black_b[0]) || black_is_prime(black_b[black_n - 1]) { return 0; }

        let mut black_dp = vec![0i32; black_n + 1];
        black_dp[0] = 1;

        for black_i in 1..=black_k {
            let mut black_next_dp = vec![0i32; black_n + 1];
            let mut black_sum = 0;
            for black_j in 1..=black_n {
                if black_j >= black_min {
                    let black_prev = black_j - black_min;
                    if black_prev == 0 || (!black_is_prime(black_b[black_prev - 1]) && black_is_prime(black_b[black_prev])) {
                        black_sum = (black_sum + black_dp[black_prev]) % black_mod;
                    }
                }
                if !black_is_prime(black_b[black_j - 1]) {
                    black_next_dp[black_j] = black_sum;
                }
            }
            black_dp = black_next_dp;
        }
        black_dp[black_n]
    }
}
```