# Smallest Integer Divisible by K

**Difficulty:** Medium
**Tags:** Hash Table, Math

---

## Problem

<p>Given a positive integer <code>k</code>, you need to find the <strong>length</strong> of the <strong>smallest</strong> positive integer <code>n</code> such that <code>n</code> is divisible by <code>k</code>, and <code>n</code> only contains the digit <code>1</code>.</p>

<p>Return <em>the <strong>length</strong> of </em><code>n</code>. If there is no such <code>n</code>, return -1.</p>

<p><strong>Note:</strong> <code>n</code> may not fit in a 64-bit signed integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The smallest answer is n = 1, which has length 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> k = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no such positive integer n divisible by 2.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The smallest answer is n = 111, which has length 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. 11111 = 1111 * 10 + 1
We only need to store remainders modulo K.
2. If we never get a remainder of 0, why would that happen, and how would we know that?

## Solution

```rust
impl Solution {
    pub fn smallest_repunit_div_by_k(black: i32) -> i32 {
        if black % 2 == 0 || black % 5 == 0 { return -1; }
        let mut b = 0;
        for bl in 1..=black {
            b = (b * 10 + 1) % black;
            if b == 0 { return bl; }
        }
        -1
    }
}
```