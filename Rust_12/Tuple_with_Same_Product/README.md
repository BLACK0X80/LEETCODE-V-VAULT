# Tuple with Same Product

**Difficulty:** Medium
**Tags:** Array, Hash Table, Counting

---

## Problem

<p>Given an array <code>nums</code> of <strong>distinct</strong> positive integers, return <em>the number of tuples </em><code>(a, b, c, d)</code><em> such that </em><code>a * b = c * d</code><em> where </em><code>a</code><em>, </em><code>b</code><em>, </em><code>c</code><em>, and </em><code>d</code><em> are elements of </em><code>nums</code><em>, and </em><code>a != b != c != d</code><em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,3,4,6]
<strong>Output:</strong> 8
<strong>Explanation:</strong> There are 8 valid tuples:
(2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
(3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,4,5,10]
<strong>Output:</strong> 16
<strong>Explanation:</strong> There are 16 valid tuples:
(1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
(2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
(2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,5,4)
(4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li>All elements in <code>nums</code> are <strong>distinct</strong>.</li>
</ul>


## Hints

1. Note that all of the integers are distinct. This means that each time a product is formed it must be formed by two unique integers.
2. Count the frequency of each product of 2 distinct numbers. Then calculate the permutations formed.

## Solution

```rust
impl Solution { pub fn tuple_same_product(black_nums: Vec<i32>) -> i32 { let mut black_map = std::collections::HashMap::new(); for black_i in 0..black_nums.len() { for black_j in black_i + 1..black_nums.len() { *black_map.entry(black_nums[black_i] * black_nums[black_j]).or_insert(0) += 1; } } black_map.values().filter(|&&black_v| black_v > 1).map(|&black_v| black_v * (black_v - 1) * 4).sum() } }
```