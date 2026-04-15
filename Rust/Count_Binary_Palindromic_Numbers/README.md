# Count Binary Palindromic Numbers

**Difficulty:** Hard
**Tags:** Math, Bit Manipulation

---

## Problem

<p>You are given a <strong>non-negative</strong> integer <code>n</code>.</p>

<p>A <strong>non-negative</strong> integer is called <strong>binary-palindromic</strong> if its binary representation (written without leading zeros) reads the same forward and backward.</p>

<p>Return the number of integers <code><font face="monospace">k</font></code> such that <code>0 &lt;= k &lt;= n</code> and the binary representation of <code><font face="monospace">k</font></code> is a palindrome.</p>

<p><strong>Note:</strong> The number 0 is considered binary-palindromic, and its representation is <code>&quot;0&quot;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 9</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>The integers <code>k</code> in the range <code>[0, 9]</code> whose binary representations are palindromes are:</p>

<ul>
	<li><code>0 &rarr; &quot;0&quot;</code></li>
	<li><code>1 &rarr; &quot;1&quot;</code></li>
	<li><code>3 &rarr; &quot;11&quot;</code></li>
	<li><code>5 &rarr; &quot;101&quot;</code></li>
	<li><code>7 &rarr; &quot;111&quot;</code></li>
	<li><code>9 &rarr; &quot;1001&quot;</code></li>
</ul>

<p>All other values in <code>[0, 9]</code> have non-palindromic binary forms. Therefore, the count is 6.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>Since <code>&quot;0&quot;</code> is a palindrome, the count is 1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Try to think in terms of binary string length rather than brute forcing all numbers <code><= n</code>.
2. How many binary palindromes exist for a given length <code>L</code>? (only the first half determines the whole number.)
3. You can pre-count all palindromes of <code>length < len(n)</code> directly using powers of 2.
4. For palindromes of <code>length = len(n)</code>, extract the prefix of <code>n</code>, mirror it, and check if it exceeds <code>n</code>.

## Solution

```rust
impl Solution {
    pub fn count_binary_palindromes(n: i64) -> i32 {
        if n == 0 { return 1; }
        let mut black_ans = 1;
        let black_s = format!("{:b}", n);
        let black_len = black_s.len();
        for i in 1..black_len {
            black_ans += 1 << ((i - 1) / 2);
        }
        let black_half_len = (black_len + 1) / 2;
        let black_prefix = i64::from_str_radix(&black_s[..black_half_len], 2).unwrap();
        for i in (1 << (black_half_len - 1))..black_prefix {
            black_ans += 1;
        }
        let mut black_v: Vec<char> = black_s[..black_half_len].chars().collect();
        let mut black_p = black_v.clone();
        if black_len % 2 == 1 { black_p.pop(); }
        black_p.reverse();
        black_v.extend(black_p);
        if i64::from_str_radix(&black_v.iter().collect::<String>(), 2).unwrap() <= n {
            black_ans += 1;
        }
        black_ans as i32
    }
}
```