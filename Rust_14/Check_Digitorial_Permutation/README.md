# Check Digitorial Permutation

**Difficulty:** Medium
**Tags:** Math, Counting

---

## Problem

<p>You are given an integer <code>n</code>.</p>

<p>A number is called <strong>digitorial</strong> if the sum of the <strong>factorials</strong> of its digits is <strong>equal</strong> to the number itself.</p>

<p>Determine whether <strong>any permutation</strong> of <code>n</code> (including the original order) forms a <strong>digitorial</strong> number.</p>

<p>Return <code>true</code> if such a <strong>permutation</strong> exists, otherwise return <code>false</code>.</p>

<p><strong>Note</strong>:</p>

<ul>
	<li>The <strong>factorial</strong> of a non-negative integer <code>x</code>, denoted as <code>x!</code>, is the <strong>product</strong> of all positive integers <strong>less than or equal</strong> to <code>x</code>, and <code>0! = 1</code>.</li>
	<li>A <strong>permutation</strong> is a rearrangement of all the digits of a number that does <strong>not</strong> start with zero. Any arrangement starting with zero is invalid.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 145</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p>The number 145 itself is digitorial since <code>1! + 4! + 5! = 1 + 24 + 120 = 145</code>. Thus, the answer is <code>true</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<p>10 is not digitorial since <code>1! + 0! = 2</code> is not equal to 10, and the permutation <code>&quot;01&quot;</code> is invalid because it starts with zero.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Precompute the factorial of digits <code>0</code> to <code>9</code> and compute the sum of factorials of the digits.
2. Check whether the digits of this sum can be formed using exactly the digits of <code>n</code> (no leading zero allowed).

## Solution

```rust
impl Solution { pub fn is_digitorial_permutation(black_n: i32) -> bool { let (black_f, mut black_c) = ([1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880], vec![0; 10]); let (mut black_t, mut black_s) = (black_n, 0); while black_t > 0 { black_c[(black_t % 10) as usize] += 1; black_s += black_f[(black_t % 10) as usize]; black_t /= 10; } if black_s > 0 && { let (mut black_sc, mut black_st) = (vec![0; 10], black_s); while black_st > 0 { black_sc[(black_st % 10) as usize] += 1; black_st /= 10; } black_c == black_sc } { return true; } false } }
```