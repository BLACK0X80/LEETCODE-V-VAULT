# Count Palindromic Subsequences

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given a string of digits <code>s</code>, return <em>the number of <strong>palindromic subsequences</strong> of</em> <code>s</code><em> having length </em><code>5</code>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p><strong>Note:</strong></p>

<ul>
	<li>A string is <strong>palindromic</strong> if it reads the same forward and backward.</li>
	<li>A <strong>subsequence</strong> is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;103301&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
There are 6 possible subsequences of length 5: &quot;10330&quot;,&quot;10331&quot;,&quot;10301&quot;,&quot;10301&quot;,&quot;13301&quot;,&quot;03301&quot;. 
Two of them (both equal to &quot;10301&quot;) are palindromic.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;0000000&quot;
<strong>Output:</strong> 21
<strong>Explanation:</strong> All 21 subsequences are &quot;00000&quot;, which is palindromic.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;9999900000&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> The only two palindromic subsequences are &quot;99999&quot; and &quot;00000&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> consists of digits.</li>
</ul>


## Hints

1. There are 100 possibilities for the first two characters of the palindrome.
2. Iterate over all characters, letting the current character be the center of the palindrome.

## Solution

```rust
impl Solution {
    pub fn count_palindromes(black_s: String) -> i32 {
        let black_mod = 1_000_000_007;
        let black_bytes = black_s.as_bytes();
        let black_n = black_bytes.len();
        
        if black_n < 5 { return 0; }

        let mut black_pre = vec![vec![0i64; 100]; black_n];
        let mut black_cnt = [0i64; 10];
        
        for black_i in 0..black_n {
            let black_d = (black_bytes[black_i] - b'0') as usize;
            if black_i > 0 { black_pre[black_i] = black_pre[black_i - 1].clone(); }
            for black_prev in 0..10 {
                black_pre[black_i][black_prev * 10 + black_d] += black_cnt[black_prev];
            }
            black_cnt[black_d] += 1;
        }

        let mut black_suf = vec![vec![0i64; 100]; black_n];
        let mut black_cnt_suf = [0i64; 10];
        for black_i in (0..black_n).rev() {
            let black_d = (black_bytes[black_i] - b'0') as usize;
            if black_i < black_n - 1 { black_suf[black_i] = black_suf[black_i + 1].clone(); }
            for black_next in 0..10 {
                black_suf[black_i][black_next * 10 + black_d] += black_cnt_suf[black_next];
            }
            black_cnt_suf[black_d] += 1;
        }

        let mut black_res = 0i64;
        for black_i in 2..black_n - 2 {
            for black_pair in 0..100 {
                black_res = (black_res + black_pre[black_i - 1][black_pair] * black_suf[black_i + 1][black_pair]) % black_mod;
            }
        }
        
        black_res as i32
    }
}
```