# Sum of Two Integers

**Difficulty:** Medium
**Tags:** Math, Bit Manipulation

---

## Problem

<p>Given two integers <code>a</code> and <code>b</code>, return <em>the sum of the two integers without using the operators</em> <code>+</code> <em>and</em> <code>-</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> a = 1, b = 2
<strong>Output:</strong> 3
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> a = 2, b = 3
<strong>Output:</strong> 5
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>-1000 &lt;= a, b &lt;= 1000</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn get_sum(mut black: i32, mut b: i32) -> i32 {
        while b != 0 {
            let bl = (black & b) << 1;
            black ^= b;
            b = bl;
        }
        black
    }
}
```