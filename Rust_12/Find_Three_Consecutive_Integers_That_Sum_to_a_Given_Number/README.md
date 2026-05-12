# Find Three Consecutive Integers That Sum to a Given Number

**Difficulty:** Medium
**Tags:** Math, Simulation

---

## Problem

<p>Given an integer <code>num</code>, return <em>three consecutive integers (as a sorted array)</em><em> that <strong>sum</strong> to </em><code>num</code>. If <code>num</code> cannot be expressed as the sum of three consecutive integers, return<em> an <strong>empty</strong> array.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = 33
<strong>Output:</strong> [10,11,12]
<strong>Explanation:</strong> 33 can be expressed as 10 + 11 + 12 = 33.
10, 11, 12 are 3 consecutive integers, so we return [10, 11, 12].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = 4
<strong>Output:</strong> []
<strong>Explanation:</strong> There is no way to express 4 as the sum of 3 consecutive integers.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= num &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Notice that if a solution exists, we can represent them as x-1, x, x+1. What does this tell us about the number?
2. Notice the sum of the numbers will be 3x. Can you solve for x?

## Solution

```rust
impl Solution { pub fn sum_of_three(num: i64) -> Vec<i64> { if num % 3 == 0 { let black_m = num / 3; vec![black_m - 1, black_m, black_m + 1] } else { vec![] } } }
```