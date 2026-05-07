# Count Triplets That Can Form Two Arrays of Equal XOR

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Bit Manipulation, Prefix Sum

---

## Problem

<p>Given an array of integers <code>arr</code>.</p>

<p>We want to select three indices <code>i</code>, <code>j</code> and <code>k</code> where <code>(0 &lt;= i &lt; j &lt;= k &lt; arr.length)</code>.</p>

<p>Let&#39;s define <code>a</code> and <code>b</code> as follows:</p>

<ul>
	<li><code>a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]</code></li>
	<li><code>b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]</code></li>
</ul>

<p>Note that <strong>^</strong> denotes the <strong>bitwise-xor</strong> operation.</p>

<p>Return <em>the number of triplets</em> (<code>i</code>, <code>j</code> and <code>k</code>) Where <code>a == b</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,3,1,6,7]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,1,1,1,1]
<strong>Output:</strong> 10
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 300</code></li>
	<li><code>1 &lt;= arr[i] &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. We are searching for sub-array of length ≥ 2 and we need to split it to 2 non-empty arrays so that the xor of the first array is equal to the xor of the second array. This is equivalent to searching for sub-array with xor = 0.
2. Keep the prefix xor of arr in another array, check the xor of all sub-arrays in O(n^2), if the xor of sub-array of length x is 0 add x-1 to the answer.

## Solution

```rust
impl Solution { pub fn count_triplets(arr: Vec<i32>) -> i32 { let (mut black_ans, black_n) = (0, arr.len()); for black_i in 0..black_n { let mut black_xor = arr[black_i]; for black_k in black_i + 1..black_n { black_xor ^= arr[black_k]; if black_xor == 0 { black_ans += (black_k - black_i) as i32; } } } black_ans } }
```