# Minimize the Maximum of Two Arrays

**Difficulty:** Medium
**Tags:** Math, Binary Search, Number Theory

---

## Problem

<p>We have two arrays <code>arr1</code> and <code>arr2</code> which are initially empty. You need to add positive integers to them such that they satisfy all the following conditions:</p>

<ul>
	<li><code>arr1</code> contains <code>uniqueCnt1</code> <strong>distinct</strong> positive integers, each of which is <strong>not divisible</strong> by <code>divisor1</code>.</li>
	<li><code>arr2</code> contains <code>uniqueCnt2</code> <strong>distinct</strong> positive integers, each of which is <strong>not divisible</strong> by <code>divisor2</code>.</li>
	<li><strong>No</strong> integer is present in both <code>arr1</code> and <code>arr2</code>.</li>
</ul>

<p>Given <code>divisor1</code>, <code>divisor2</code>, <code>uniqueCnt1</code>, and <code>uniqueCnt2</code>, return <em>the <strong>minimum possible maximum</strong> integer that can be present in either array</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> divisor1 = 2, divisor2 = 7, uniqueCnt1 = 1, uniqueCnt2 = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> 
We can distribute the first 4 natural numbers into arr1 and arr2.
arr1 = [1] and arr2 = [2,3,4].
We can see that both arrays satisfy all the conditions.
Since the maximum value is 4, we return it.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> divisor1 = 3, divisor2 = 5, uniqueCnt1 = 2, uniqueCnt2 = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
Here arr1 = [1,2], and arr2 = [3] satisfy all conditions.
Since the maximum value is 3, we return it.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> divisor1 = 2, divisor2 = 4, uniqueCnt1 = 8, uniqueCnt2 = 2
<strong>Output:</strong> 15
<strong>Explanation:</strong> 
Here, the final possible arrays can be arr1 = [1,3,5,7,9,11,13,15], and arr2 = [2,6].
It can be shown that it is not possible to obtain a lower maximum satisfying all conditions. 
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= divisor1, divisor2 &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= uniqueCnt1, uniqueCnt2 &lt; 10<sup>9</sup></code></li>
	<li><code>2 &lt;= uniqueCnt1 + uniqueCnt2 &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use binary search to find smallest maximum element.
2. Add numbers divisible by x in nums2 and vice versa.

## Solution

```rust
impl Solution { pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 { let (mut black_l, mut black_r, black_g) = (1i64, 2_000_000_000i64, (divisor1 as i64 * divisor2 as i64) / { let (mut a, mut b) = (divisor1 as i64, divisor2 as i64); while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); } a }); while black_l < black_r { let black_m = (black_l + black_r) / 2; if black_m - black_m / divisor1 as i64 >= unique_cnt1 as i64 && black_m - black_m / divisor2 as i64 >= unique_cnt2 as i64 && black_m - black_m / black_g >= (unique_cnt1 + unique_cnt2) as i64 { black_r = black_m; } else { black_l = black_m + 1; } } black_l as i32 } }
```