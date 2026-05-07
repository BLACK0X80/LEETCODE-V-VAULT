# Restore The Array

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>A program was supposed to print an array of integers. The program forgot to print whitespaces and the array is printed as a string of digits <code>s</code> and all we know is that all integers in the array were in the range <code>[1, k]</code> and there are no leading zeros in the array.</p>

<p>Given the string <code>s</code> and the integer <code>k</code>, return <em>the number of the possible arrays that can be printed as </em><code>s</code><em> using the mentioned program</em>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;1000&quot;, k = 10000
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only possible array is [1000]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;1000&quot;, k = 10
<strong>Output:</strong> 0
<strong>Explanation:</strong> There cannot be an array that was printed this way and has all integer &gt;= 1 and &lt;= 10.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;1317&quot;, k = 2000
<strong>Output:</strong> 8
<strong>Explanation:</strong> Possible arrays are [1317],[131,7],[13,17],[1,317],[13,1,7],[1,31,7],[1,3,17],[1,3,1,7]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of only digits and does not contain leading zeros.</li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use dynamic programming. Build an array dp where dp[i] is the number of ways you can divide the string starting from index i to the end.
2. Keep in mind that the answer is modulo 10^9 + 7 and take the mod for each operation.

## Solution

```rust
impl Solution {
    pub fn number_of_arrays(black_s: String, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let black_bytes = black_s.as_bytes();
        let black_n = black_bytes.len();
        let black_k64 = black_k as i64;
        let mut black_dp = vec![0; black_n + 1];
        black_dp[black_n] = 1;

        for black_i in (0..black_n).rev() {
            if black_bytes[black_i] == b'0' { continue; }
            let mut black_num = 0i64;
            for black_j in black_i..black_n {
                black_num = black_num * 10 + (black_bytes[black_j] - b'0') as i64;
                if black_num > black_k64 { break; }
                black_dp[black_i] = (black_dp[black_i] + black_dp[black_j + 1]) % black_mod;
            }
        }
        black_dp[0]
    }
}
```