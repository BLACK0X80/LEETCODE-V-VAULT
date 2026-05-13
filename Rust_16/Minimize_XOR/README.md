# Minimize XOR

**Difficulty:** Medium
**Tags:** Greedy, Bit Manipulation

---

## Problem

<p>Given two positive integers <code>num1</code> and <code>num2</code>, find the positive integer <code>x</code> such that:</p>

<ul>
	<li><code>x</code> has the same number of set bits as <code>num2</code>, and</li>
	<li>The value <code>x XOR num1</code> is <strong>minimal</strong>.</li>
</ul>

<p>Note that <code>XOR</code> is the bitwise XOR operation.</p>

<p>Return <em>the integer </em><code>x</code>. The test cases are generated such that <code>x</code> is <strong>uniquely determined</strong>.</p>

<p>The number of <strong>set bits</strong> of an integer is the number of <code>1</code>&#39;s in its binary representation.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num1 = 3, num2 = 5
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The binary representations of num1 and num2 are 0011 and 0101, respectively.
The integer <strong>3</strong> has the same number of set bits as num2, and the value <code>3 XOR 3 = 0</code> is minimal.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num1 = 1, num2 = 12
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The binary representations of num1 and num2 are 0001 and 1100, respectively.
The integer <strong>3</strong> has the same number of set bits as num2, and the value <code>3 XOR 1 = 2</code> is minimal.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num1, num2 &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. To arrive at a small xor, try to turn off some bits from num1
2. If there are still left bits to set, try to set them from the least significant bit

## Solution

```rust
impl Solution { pub fn minimize_xor(black_n1: i32, black_n2: i32) -> i32 { let (mut black_target, mut black_res) = (black_n2.count_ones() as i32, 0); for black_i in (0..31).rev() { if (black_n1 & (1 << black_i)) != 0 && black_target > 0 { black_res |= (1 << black_i); black_target -= 1; } } for black_i in 0..31 { if (black_res & (1 << black_i)) == 0 && black_target > 0 { black_res |= (1 << black_i); black_target -= 1; } } black_res } }
```