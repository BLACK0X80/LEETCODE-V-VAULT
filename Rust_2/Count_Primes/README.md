# Count Primes

**Difficulty:** Medium
**Tags:** Array, Math, Enumeration, Number Theory

---

## Problem

<p>Given an integer <code>n</code>, return <em>the number of prime numbers that are strictly less than</em> <code>n</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 0
<strong>Output:</strong> 0
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 5 * 10<sup>6</sup></code></li>
</ul>


## Hints

1. Checking all the integers in the range [1, n - 1] is not efficient. Think about a better approach.
2. Since most of the numbers are not primes, we need a fast approach to exclude the non-prime integers.
3. Use Sieve of Eratosthenes.

## Solution

```rust
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n < 2 { return 0; }
        let mut black = vec![true; n];
        black[0] = false; black[1] = false;
        let mut b = 2;
        while b * b < n {
            if black[b] { let mut bl = b*b; while bl < n { black[bl] = false; bl += b; } }
            b += 1;
        }
        black.iter().filter(|&&b| b).count() as i32
    }
}
```