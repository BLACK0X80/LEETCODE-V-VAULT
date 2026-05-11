# Super Pow

**Difficulty:** Medium
**Tags:** Math, Divide and Conquer

---

## Problem

<p>Your task is to calculate <code>a<sup>b</sup></code> mod <code>1337</code> where <code>a</code> is a positive integer and <code>b</code> is an extremely large positive integer given in the form of an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> a = 2, b = [3]
<strong>Output:</strong> 8
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> a = 2, b = [1,0]
<strong>Output:</strong> 1024
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> a = 1, b = [4,3,3,8,5,2]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= a &lt;= 2<sup>31</sup> - 1</code></li>
	<li><code>1 &lt;= b.length &lt;= 2000</code></li>
	<li><code>0 &lt;= b[i] &lt;= 9</code></li>
	<li><code>b</code> does not contain leading zeros.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn super_pow(black_a: i32, black_b: Vec<i32>) -> i32 {
        black_b.iter().fold(1, |black_res, &black_digit| { let (black_a_m, black_m) = (black_a % 1337, 1337); let black_pow = |mut black_base: i32, mut black_exp: i32| { let mut black_p = 1; while black_exp > 0 { if black_exp % 2 == 1 { black_p = (black_p * black_base) % black_m; } black_base = (black_base * black_base) % black_m; black_exp /= 2; } black_p }; (black_pow(black_res, 10) * black_pow(black_a_m, black_digit)) % black_m })
    }
}
```