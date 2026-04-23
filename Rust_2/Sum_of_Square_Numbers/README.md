# Sum of Square Numbers

**Difficulty:** Medium
**Tags:** Math, Two Pointers, Binary Search

---

## Problem

<p>Given a non-negative integer <code>c</code>, decide whether there&#39;re two integers <code>a</code> and <code>b</code> such that <code>a<sup>2</sup> + b<sup>2</sup> = c</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> c = 5
<strong>Output:</strong> true
<strong>Explanation:</strong> 1 * 1 + 2 * 2 = 5
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> c = 3
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= c &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let (mut l, mut r) = (0i64, (c as i64).isqrt());
        while l <= r {
            let s = l * l + r * r;
            if s == c as i64 { return true; }
            else if s < c as i64 { l += 1; } else { r -= 1; }
        }
        false
    }
}
```