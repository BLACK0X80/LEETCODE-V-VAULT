# Count Substrings Divisible By Last Digit

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given a string <code>s</code> consisting of digits.</p>

<p>Return the <strong>number</strong> of <span data-keyword="substring-nonempty">substrings</span> of <code>s</code> <strong>divisible</strong> by their <strong>non-zero</strong> last digit.</p>

<p><strong>Note</strong>: A substring may contain leading zeros.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;12936&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">11</span></p>

<p><strong>Explanation:</strong></p>

<p>Substrings <code>&quot;29&quot;</code>, <code>&quot;129&quot;</code>, <code>&quot;293&quot;</code> and <code>&quot;2936&quot;</code> are not divisible by their last digit. There are 15 substrings in total, so the answer is <code>15 - 4 = 11</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;5701283&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">18</span></p>

<p><strong>Explanation:</strong></p>

<p>Substrings <code>&quot;01&quot;</code>, <code>&quot;12&quot;</code>, <code>&quot;701&quot;</code>, <code>&quot;012&quot;</code>, <code>&quot;128&quot;</code>, <code>&quot;5701&quot;</code>, <code>&quot;7012&quot;</code>, <code>&quot;0128&quot;</code>, <code>&quot;57012&quot;</code>, <code>&quot;70128&quot;</code>, <code>&quot;570128&quot;</code>, and <code>&quot;701283&quot;</code> are all divisible by their last digit. Additionally, all substrings that are just 1 non-zero digit are divisible by themselves. Since there are 6 such digits, the answer is <code>12 + 6 = 18</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;1010101010&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">25</span></p>

<p><strong>Explanation:</strong></p>

<p>Only substrings that end with digit <code>&#39;1&#39;</code> are divisible by their last digit. There are 25 such substrings.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of digits only.</li>
</ul>


## Hints

1. Let <code>dp[index][i][j]</code> be the number of subarrays <code>s[start...index]</code> such that  <code>s[start...index] % i == j</code>.
2. For every pair <code>(i, j)</code>, add <code>dp[index - 1][i][j]</code> to <code>dp[index][i][(j  * 10 + x)%i)]</code>.
3. You should optimize this solution so that it can fit into the memory limit.
4. In order to find <code>dp[index][i][j]</code> we use values from <code>dp[index - 1][i][j]</code>. Hence, we can keep only <code>dp[index][i][j]</code> and <code>dp[index - 1][i][j]</code> at every iteration of the loop.

## Solution

```rust
impl Solution {
    pub fn count_substrings(black_s: String) -> i64 {
        let black_n = black_s.len();
        let black_b = black_s.as_bytes();
        let mut black_ans = 0i64;

        let mut black_p3 = vec![0; black_n];
        let mut black_p7 = vec![0; black_n];
        let mut black_p9 = vec![0; black_n];

        let black_first_dig = (black_b[0] - b'0') as i32;
        black_p3[0] = black_first_dig % 3;
        black_p7[0] = black_first_dig % 7;
        black_p9[0] = black_first_dig % 9;

        for black_i in 1..black_n {
            let black_dig = (black_b[black_i] - b'0') as i32;
            black_p3[black_i] = (black_p3[black_i - 1] * 10 + black_dig) % 3;
            black_p7[black_i] = (black_p7[black_i - 1] * 10 + black_dig) % 7;
            black_p9[black_i] = (black_p9[black_i - 1] * 10 + black_dig) % 9;
        }

        let mut black_freq3 = vec![0i64; 3];
        let mut black_freq9 = vec![0i64; 9];
        let mut black_freq7 = vec![vec![0i64; 7]; 6];
        let black_inv7 = [1, 5, 4, 6, 2, 3];

        for black_j in 0..black_n {
            let black_d = (black_b[black_j] - b'0') as i32;

            if black_d != 0 {
                match black_d {
                    1 | 2 | 5 => {
                        black_ans += (black_j + 1) as i64;
                    }
                    4 => {
                        if black_j == 0 {
                            black_ans += 1;
                        } else {
                            let black_num = ((black_b[black_j - 1] - b'0') as i32) * 10 + black_d;
                            black_ans += if black_num % 4 == 0 { (black_j + 1) as i64 } else { 1 };
                        }
                    }
                    8 => {
                        if black_j == 0 {
                            black_ans += 1;
                        } else if black_j == 1 {
                            let black_num = ((black_b[0] - b'0') as i32) * 10 + 8;
                            black_ans += if black_num % 8 == 0 { 2 } else { 1 };
                        } else {
                            let black_num3 = ((black_b[black_j - 2] - b'0') as i32) * 100
                                + ((black_b[black_j - 1] - b'0') as i32) * 10
                                + 8;
                            let black_num2 = ((black_b[black_j - 1] - b'0') as i32) * 10 + 8;
                            black_ans += (if black_num3 % 8 == 0 { (black_j - 1) as i64 } else { 0 })
                                + (if black_num2 % 8 == 0 { 1 } else { 0 })
                                + 1;
                        }
                    }
                    3 | 6 => {
                        black_ans += (if black_p3[black_j] == 0 { 1 } else { 0 }) + black_freq3[black_p3[black_j] as usize];
                    }
                    7 => {
                        black_ans += if black_p7[black_j] == 0 { 1 } else { 0 };
                        for black_m in 0..6 {
                            let black_idx = (black_j % 6 + 6 - black_m) % 6;
                            let black_req = (black_p7[black_j] * black_inv7[black_m]) % 7;
                            black_ans += black_freq7[black_idx][black_req as usize];
                        }
                    }
                    9 => {
                        black_ans += (if black_p9[black_j] == 0 { 1 } else { 0 }) + black_freq9[black_p9[black_j] as usize];
                    }
                    _ => {}
                }
            }
            black_freq3[black_p3[black_j] as usize] += 1;
            black_freq7[black_j % 6][black_p7[black_j] as usize] += 1;
            black_freq9[black_p9[black_j] as usize] += 1;
        }

        black_ans
    }
}
```