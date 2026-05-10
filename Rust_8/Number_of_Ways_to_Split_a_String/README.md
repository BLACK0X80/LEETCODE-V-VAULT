# Number of Ways to Split a String

**Difficulty:** Medium
**Tags:** Math, String

---

## Problem

<p>Given a binary string <code>s</code>, you can split <code>s</code> into 3 <strong>non-empty</strong> strings <code>s1</code>, <code>s2</code>, and <code>s3</code> where <code>s1 + s2 + s3 = s</code>.</p>

<p>Return the number of ways <code>s</code> can be split such that the number of ones is the same in <code>s1</code>, <code>s2</code>, and <code>s3</code>. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;10101&quot;
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are four ways to split s in 3 parts where each part contain the same number of letters &#39;1&#39;.
&quot;1|010|1&quot;
&quot;1|01|01&quot;
&quot;10|10|1&quot;
&quot;10|1|01&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;1001&quot;
<strong>Output:</strong> 0
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;0000&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three ways to split s in 3 parts.
&quot;0|0|00&quot;
&quot;0|00|0&quot;
&quot;00|0|0&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
</ul>


## Hints

1. There is no way if the sum (number of '1's) is not divisible by the number of splits. So sum%3 should be 0.
2. Preffix s1 , and suffix s3 should have sum/3 characters '1'.
3. Follow up: Can you generalize the problem with numbers between [-10^9, 10^9] such the sum between subarrays s1, s2, s3 are the same?

## Solution

```rust
impl Solution { pub fn num_ways(s: String) -> i32 { let (black_mod, black_b) = (1_000_000_007i64, s.as_bytes()); let black_ones: Vec<usize> = black_b.iter().enumerate().filter(|&(_, &c)| c == b'1').map(|(i, _)| i).collect(); let black_n = black_b.len() as i64; if black_ones.len() == 0 { return (((black_n - 1) * (black_n - 2) / 2) % black_mod) as i32; } if black_ones.len() % 3 != 0 { return 0; } let black_k = black_ones.len() / 3; let black_w1 = (black_ones[black_k] - black_ones[black_k - 1]) as i64; let black_w2 = (black_ones[2 * black_k] - black_ones[2 * black_k - 1]) as i64; (black_w1 * black_w2 % black_mod) as i32 } }
```