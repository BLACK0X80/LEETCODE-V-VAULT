# Consecutive Numbers Sum

**Difficulty:** Hard
**Tags:** Math, Enumeration

---

## Problem

<p>Given an integer <code>n</code>, return <em>the number of ways you can write </em><code>n</code><em> as the sum of consecutive positive integers.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> 5 = 2 + 3
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 9
<strong>Output:</strong> 3
<strong>Explanation:</strong> 9 = 4 + 5 = 2 + 3 + 4
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 15
<strong>Output:</strong> 4
<strong>Explanation:</strong> 15 = 8 + 7 = 4 + 5 + 6 = 1 + 2 + 3 + 4 + 5
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn consecutive_numbers_sum(black1: i32) -> i32 {
        let mut black2 = 0;
        let mut black3 = 1;
        while black3 * (black3 - 1) / 2 < black1 {
            if (black1 - black3 * (black3 - 1) / 2) % black3 == 0 { black2 += 1; }
            black3 += 1;
        }
        black2
    }
}
```