# String Transformation

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming, String Matching

---

## Problem

<p>You are given two strings <code>s</code> and <code>t</code> of equal length <code>n</code>. You can perform the following operation on the string <code>s</code>:</p>

<ul>
	<li>Remove a <strong>suffix</strong> of <code>s</code> of length <code>l</code> where <code>0 &lt; l &lt; n</code> and append it at the start of <code>s</code>.<br />
	For example, let <code>s = &#39;abcd&#39;</code> then in one operation you can remove the suffix <code>&#39;cd&#39;</code> and append it in front of <code>s</code> making <code>s = &#39;cdab&#39;</code>.</li>
</ul>

<p>You are also given an integer <code>k</code>. Return <em>the number of ways in which </em><code>s</code> <em>can be transformed into </em><code>t</code><em> in <strong>exactly</strong> </em><code>k</code><em> operations.</em></p>

<p>Since the answer can be large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcd&quot;, t = &quot;cdab&quot;, k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
First way:
In first operation, choose suffix from index = 3, so resulting s = &quot;dabc&quot;.
In second operation, choose suffix from index = 3, so resulting s = &quot;cdab&quot;.

Second way:
In first operation, choose suffix from index = 1, so resulting s = &quot;bcda&quot;.
In second operation, choose suffix from index = 1, so resulting s = &quot;cdab&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ababab&quot;, t = &quot;ababab&quot;, k = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
First way:
Choose suffix from index = 2, so resulting s = &quot;ababab&quot;.

Second way:
Choose suffix from index = 4, so resulting s = &quot;ababab&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= s.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>15</sup></code></li>
	<li><code>s.length == t.length</code></li>
	<li><code>s</code> and <code>t</code> consist of only lowercase English alphabets.</li>
</ul>


## Hints

1. String <code>t</code> can be only constructed if it is a rotated version of string <code>s</code>.
2. Use KMP algorithm or Z algorithm to find the number of indices from where <code>s</code> is equal to <code>t</code>.
3. Use Dynamic Programming to count the number of ways.

## Solution

```rust
impl Solution {
    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = s.len();

        fn z_func(s: &[u8]) -> Vec<usize> {
            let n = s.len();
            let mut z = vec![0usize; n];
            let (mut l, mut r) = (0usize, 0usize);
            for i in 1..n {
                if i <= r && z[i - l] <= r - i { z[i] = z[i - l]; }
                else {
                    z[i] = if r > i { r - i + 1 } else { 0 };
                    while i + z[i] < n && s[i + z[i]] == s[z[i]] { z[i] += 1; }
                }
                if i + z[i] - 1 > r { l = i; r = i + z[i] - 1; }
            }
            z
        }

        fn mat_mul(a: &[[i64;2];2], b: &[[i64;2];2]) -> [[i64;2];2] {
            const MOD: i64 = 1_000_000_007;
            let mut c = [[0i64;2];2];
            for i in 0..2 { for j in 0..2 { for l in 0..2 {
                c[i][j] = (c[i][j] + a[i][l] * b[l][j]) % MOD;
            }}}
            c
        }

        fn mat_pow(mut base: [[i64;2];2], mut p: i64) -> [[i64;2];2] {
            let mut res = [[1,0],[0,1]];
            while p > 0 {
                if p & 1 == 1 { res = mat_mul(&res, &base); }
                base = mat_mul(&base, &base);
                p >>= 1;
            }
            res
        }

        let dp = mat_pow([[0, 1], [n as i64 - 1, n as i64 - 2]], k)[0];

        let mut combined = s.as_bytes().to_vec();
        combined.extend_from_slice(t.as_bytes());
        combined.extend_from_slice(t.as_bytes());
        let z = z_func(&combined);

        let mut result = 0i64;
        for i in n..2*n {
            if z[i] >= n {
                let idx = if i - n == 0 { 0 } else { 1 };
                result = (result + dp[idx]) % MOD;
            }
        }

        result as i32
    }
}
```