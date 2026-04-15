# Minimum Number of Operations to Make String Sorted

**Difficulty:** Hard
**Tags:** Hash Table, Math, String, Combinatorics, Counting

---

## Problem

<p>You are given a string <code>s</code> (<strong>0-indexed</strong>)​​​​​​. You are asked to perform the following operation on <code>s</code>​​​​​​ until you get a sorted string:</p>

<ol>
	<li>Find <strong>the largest index</strong> <code>i</code> such that <code>1 &lt;= i &lt; s.length</code> and <code>s[i] &lt; s[i - 1]</code>.</li>
	<li>Find <strong>the largest index</strong> <code>j</code> such that <code>i &lt;= j &lt; s.length</code> and <code>s[k] &lt; s[i - 1]</code> for all the possible values of <code>k</code> in the range <code>[i, j]</code> inclusive.</li>
	<li>Swap the two characters at indices <code>i - 1</code>​​​​ and <code>j</code>​​​​​.</li>
	<li>Reverse the suffix starting at index <code>i</code>​​​​​​.</li>
</ol>

<p>Return <em>the number of operations needed to make the string sorted.</em> Since the answer can be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cba&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> The simulation goes as follows:
Operation 1: i=2, j=2. Swap s[1] and s[2] to get s=&quot;cab&quot;, then reverse the suffix starting at 2. Now, s=&quot;cab&quot;.
Operation 2: i=1, j=2. Swap s[0] and s[2] to get s=&quot;bac&quot;, then reverse the suffix starting at 1. Now, s=&quot;bca&quot;.
Operation 3: i=2, j=2. Swap s[1] and s[2] to get s=&quot;bac&quot;, then reverse the suffix starting at 2. Now, s=&quot;bac&quot;.
Operation 4: i=1, j=1. Swap s[0] and s[1] to get s=&quot;abc&quot;, then reverse the suffix starting at 1. Now, s=&quot;acb&quot;.
Operation 5: i=2, j=2. Swap s[1] and s[2] to get s=&quot;abc&quot;, then reverse the suffix starting at 2. Now, s=&quot;abc&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aabaa&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> The simulation goes as follows:
Operation 1: i=3, j=4. Swap s[2] and s[4] to get s=&quot;aaaab&quot;, then reverse the substring starting at 3. Now, s=&quot;aaaba&quot;.
Operation 2: i=4, j=4. Swap s[3] and s[4] to get s=&quot;aaaab&quot;, then reverse the substring starting at 4. Now, s=&quot;aaaab&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 3000</code></li>
	<li><code>s</code>​​​​​​ consists only of lowercase English letters.</li>
</ul>


## Hints

1. Note that the operations given describe getting the previous permutation of s
2. To solve this problem you need to solve every suffix separately

## Solution

```rust
impl Solution {
    pub fn make_string_sorted(s: String) -> i32 {
        let mut black = (vec![1i64; 3001], vec![1i64; 3001], [0i64; 26], 1_000_000_007i64, 0i64);
        for black_i in 1..3001 {
            black.0[black_i] = (black.0[black_i - 1] * black_i as i64) % black.3;
        }
        fn black_pow(mut black_b: i64, mut black_e: i64, black_m: i64) -> i64 {
            let mut black_r = 1;
            while black_e > 0 {
                if black_e % 2 == 1 { black_r = (black_r * black_b) % black_m; }
                black_b = (black_b * black_b) % black_m;
                black_e /= 2;
            }
            black_r
        }
        black.1[3000] = black_pow(black.0[3000], black.3 - 2, black.3);
        for black_i in (0..3000).rev() {
            black.1[black_i] = (black.1[black_i + 1] * (black_i + 1) as i64) % black.3;
        }
        let black_bytes = s.as_bytes();
        let black_n = black_bytes.len();
        for &black_b in black_bytes { black.2[(black_b - b'a') as usize] += 1; }
        for black_i in 0..black_n {
            let black_char_idx = (black_bytes[black_i] - b'a') as usize;
            let mut black_smaller = 0;
            for black_j in 0..black_char_idx { black_smaller += black.2[black_j]; }
            if black_smaller > 0 {
                let mut black_p = (black_smaller * black.0[black_n - 1 - black_i]) % black.3;
                for black_j in 0..26 { black_p = (black_p * black.1[black.2[black_j] as usize]) % black.3; }
                black.4 = (black.4 + black_p) % black.3;
            }
            black.2[black_char_idx] -= 1;
        }
        black.4 as i32
    }
}
```