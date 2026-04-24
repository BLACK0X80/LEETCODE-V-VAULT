# Number of Ways to Separate Numbers

**Difficulty:** Hard
**Tags:** String, Dynamic Programming, Suffix Array

---

## Problem

<p>You wrote down many <strong>positive</strong> integers in a string called <code>num</code>. However, you realized that you forgot to add commas to seperate the different numbers. You remember that the list of integers was <strong>non-decreasing</strong> and that <strong>no</strong> integer had leading zeros.</p>

<p>Return <em>the <strong>number of possible lists of integers</strong> that you could have written down to get the string </em><code>num</code>. Since the answer may be large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;327&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> You could have written down the numbers:
3, 27
327
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;094&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong> No numbers can have leading zeros and all numbers must be positive.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;0&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong> No numbers can have leading zeros and all numbers must be positive.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 3500</code></li>
	<li><code>num</code> consists of digits <code>&#39;0&#39;</code> through <code>&#39;9&#39;</code>.</li>
</ul>


## Hints

1. If we know the current number has d digits, how many digits can the previous number have?
2. Is there a quick way of calculating the number of possibilities for the previous number if we know that it must have less than or equal to d digits? Try to do some pre-processing.

## Solution

```rust
impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let num = num.as_bytes();
        let n = num.len();
        let mod_val = 1_000_000_007i64;
        let mut black_lcp = vec![0u16; (n + 1) * (n + 1)];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                let idx = i * (n + 1) + j;
                black_lcp[idx] = if num[i] == num[j] { black_lcp[idx + n + 2] + 1 } else { 0 };
            }
        }
        let mut black_a = vec![0i32; n * n];
        let mut black_b = vec![0i32; n * n];
        for i in (0..n).rev() {
            if num[i] == b'0' { continue; }
            let mut black_sum = 0i64;
            let row_off = i * n;
            for j in (i..n).rev() {
                let idx = row_off + j;
                if j == n - 1 {
                    black_a[idx] = 1;
                } else {
                    let len = j - i + 1;
                    let first = j + 1;
                    let last = first + len;
                    let mut black_cnt = 0i64;
                    if last <= n {
                        let common = black_lcp[i * (n + 1) + first] as usize;
                        if common >= len || num[i + common] <= num[first + common] {
                            black_cnt += black_a[first * n + last - 1] as i64;
                        }
                    }
                    if last < n {
                        black_cnt += black_b[first * n + last] as i64;
                    }
                    black_a[idx] = (black_cnt % mod_val) as i32;
                }
                black_sum += black_a[idx] as i64;
                if black_sum >= mod_val { black_sum -= mod_val; }
                black_b[idx] = black_sum as i32;
            }
        }
        black_b[0]
    }
}
```