# Maximize Number of Nice Divisors

**Difficulty:** Hard
**Tags:** Math, Recursion, Number Theory

---

## Problem

<p>You are given a positive integer <code>primeFactors</code>. You are asked to construct a positive integer <code>n</code> that satisfies the following conditions:</p>

<ul>
  <li>The number of prime factors of <code>n</code> (not necessarily distinct) is <strong>at most</strong> <code>primeFactors</code>.</li>
  <li>The number of nice divisors of <code>n</code> is maximized. Note that a divisor of <code>n</code> is <strong>nice</strong> if it is divisible by every prime factor of <code>n</code>. For example, if <code>n = 12</code>, then its prime factors are <code>[2,2,3]</code>, then <code>6</code> and <code>12</code> are nice divisors, while <code>3</code> and <code>4</code> are not.</li>
</ul>

<p>Return <em>the number of nice divisors of</em> <code>n</code>. Since that number can be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>Note that a prime number is a natural number greater than <code>1</code> that is not a product of two smaller natural numbers. The prime factors of a number <code>n</code> is a list of prime numbers such that their product equals <code>n</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> primeFactors = 5
<strong>Output:</strong> 6
<strong>Explanation:</strong> 200 is a valid value of n.
It has 5 prime factors: [2,2,2,5,5], and it has 6 nice divisors: [10,20,40,50,100,200].
There is not other value of n that has at most 5 prime factors and more nice divisors.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> primeFactors = 8
<strong>Output:</strong> 18
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= primeFactors &lt;= 10<sup>9</sup></code></li>
</ul>

## Hints

1. The number of nice divisors is equal to the product of the count of each prime factor. Then the problem is reduced to: given n, find a sequence of numbers whose sum equals n and whose product is maximized.
2. This sequence can have no numbers that are larger than 4. Proof: if it contains a number x that is larger than 4, then you can replace x with floor(x/2) and ceil(x/2), and floor(x/2) * ceil(x/2) > x. You can also replace 4s with two 2s. Hence, there will always be optimal solutions with only 2s and 3s.
3. If there are three 2s, you can replace them with two 3s to get a better product. Hence, you'll never have more than two 2s.
4. Keep adding 3s as long as n ≥ 5.

## Solution

```rust
impl Solution {
    pub fn max_nice_divisors(black1: i32) -> i32 {
        if black1 <= 3 { return black1; }
        let black2 = 1_000_000_007i64;
        match black1 % 3 {
            0 => Self::black_pow(3, (black1 / 3) as i64, black2) as i32,
            1 => (Self::black_pow(3, (black1 / 3 - 1) as i64, black2) * 4 % black2) as i32,
            _ => (Self::black_pow(3, (black1 / 3) as i64, black2) * 2 % black2) as i32,
        }
    }

    fn black_pow(mut b: i64, mut e: i64, m: i64) -> i64 {
        let mut r = 1;
        b %= m;
        while e > 0 {
            if e % 2 == 1 { r = (r * b) % m; }
            b = (b * b) % m;
            e /= 2;
        }
        r
    }
}
```