# Number of Digit One

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Recursion

---

## Problem

<p>Given an integer <code>n</code>, count <em>the total number of digit </em><code>1</code><em> appearing in all non-negative integers less than or equal to</em> <code>n</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 13
<strong>Output:</strong> 6
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 0
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Beware of overflow.

## Solution

```rust
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let (mut black, mut b, n) = (0i64, 1i64, n as i64);
        while b <= n {
            let (hi, cur, lo) = (n/(b*10), (n/b)%10, n%b);
            black += hi * b + match cur { 0 => 0, 1 => lo+1, _ => b };
            b *= 10;
        }
        black as i32
    }
}
```