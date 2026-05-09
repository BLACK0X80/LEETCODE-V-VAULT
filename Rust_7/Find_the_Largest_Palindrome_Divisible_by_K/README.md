# Find the Largest Palindrome Divisible by K

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming, Greedy, Number Theory

---

## Problem

<p>You are given two <strong>positive</strong> integers <code>n</code> and <code>k</code>.</p>

<p>An integer <code>x</code> is called <strong>k-palindromic</strong> if:</p>

<ul>
	<li><code>x</code> is a <span data-keyword="palindrome-integer">palindrome</span>.</li>
	<li><code>x</code> is divisible by <code>k</code>.</li>
</ul>

<p>Return the<strong> largest</strong> integer having <code>n</code> digits (as a string) that is <strong>k-palindromic</strong>.</p>

<p><strong>Note</strong> that the integer must <strong>not</strong> have leading zeros.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, k = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;595&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>595 is the largest k-palindromic integer with 3 digits.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1, k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;8&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>4 and 8 are the only k-palindromic integers with 1 digit.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, k = 6</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;89898&quot;</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 9</code></li>
</ul>


## Hints

1. It must have a solution since we can have all digits equal to <code>k</code>.
2. Use string dp, store modulus along with length of number currently formed.
3. Is it possible to solve greedily using divisibility rules?

## Solution

```rust
impl Solution {
    pub fn largest_palindrome(black_n: i32, black_k: i32) -> String {
        let black_n = black_n as usize;
        let mut black_res = vec![b'9'; black_n];
        match black_k {
            1 | 3 | 9 => {}
            2 => { black_res[0] = b'8'; black_res[black_n - 1] = b'8'; }
            4 => {
                for black_i in 0..black_n.min(2) { black_res[black_i] = b'8'; black_res[black_n - 1 - black_i] = b'8'; }
            }
            5 => { black_res[0] = b'5'; black_res[black_n - 1] = b'5'; }
            6 => {
                if black_n == 1 { return "6".to_string(); }
                if black_n == 2 { return "66".to_string(); }
                black_res[0] = b'8'; black_res[black_n - 1] = b'8';
                if black_n % 2 == 1 { black_res[black_n / 2] = b'8'; }
                else { black_res[black_n / 2] = b'7'; black_res[black_n / 2 - 1] = b'7'; }
            }
            7 => {
                for black_d in (b'0'..=b'9').rev() {
                    let mut black_test = black_res.clone();
                    black_test[black_n / 2] = black_d;
                    black_test[(black_n - 1) / 2] = black_d;
                    let mut black_rem = 0;
                    for &black_b in &black_test { black_rem = (black_rem * 10 + (black_b - b'0') as i32) % 7; }
                    if black_rem == 0 { black_res = black_test; break; }
                }
            }
            8 => {
                for black_i in 0..black_n.min(3) { black_res[black_i] = b'8'; black_res[black_n - 1 - black_i] = b'8'; }
            }
            _ => {}
        }
        let bravexuneth = String::from_utf8(black_res).unwrap();
        bravexuneth
    }
}
```