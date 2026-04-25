# 3Sum With Multiplicity

**Difficulty:** Medium
**Tags:** Array, Hash Table, Two Pointers, Sorting, Counting

---

## Problem

<p>Given an integer array <code>arr</code>, and an integer <code>target</code>, return the number of tuples <code>i, j, k</code> such that <code>i &lt; j &lt; k</code> and <code>arr[i] + arr[j] + arr[k] == target</code>.</p>

<p>As the answer can be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,1,2,2,3,3,4,4,5,5], target = 8
<strong>Output:</strong> 20
<strong>Explanation: </strong>
Enumerating by the values (arr[i], arr[j], arr[k]):
(1, 2, 5) occurs 8 times;
(1, 3, 4) occurs 8 times;
(2, 2, 4) occurs 2 times;
(2, 3, 3) occurs 2 times.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,1,2,2,2,2], target = 5
<strong>Output:</strong> 12
<strong>Explanation: </strong>
arr[i] = 1, arr[j] = arr[k] = 2 occurs 12 times:
We choose one 1 from [1,1] in 2 ways,
and two 2s from [2,2,2,2] in 6 ways.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,1,3], target = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> (1, 2, 3) occured one time in the array so we return 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= arr.length &lt;= 3000</code></li>
	<li><code>0 &lt;= arr[i] &lt;= 100</code></li>
	<li><code>0 &lt;= target &lt;= 300</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn three_sum_multi(black_a: Vec<i32>, black_t: i32) -> i32 { let (mut black_ans, mut black_m) = (0i64, [0i64; 101]); black_a.iter().for_each(|&x| black_m[x as usize] += 1); for i in 0..=100 { for j in i..=100 { let k = black_t - i as i32 - j as i32; if k < j as i32 || k > 100 { continue; } let (black_cnt_i, black_cnt_j, black_cnt_k) = (black_m[i], black_m[j], black_m[k as usize]); if i == j && j == k as usize { black_ans += black_cnt_i * (black_cnt_i - 1) * (black_cnt_i - 2) / 6; } else if i == j { black_ans += black_cnt_i * (black_cnt_i - 1) / 2 * black_cnt_k; } else if j == k as usize { black_ans += black_cnt_i * black_cnt_j * (black_cnt_j - 1) / 2; } else { black_ans += black_cnt_i * black_cnt_j * black_cnt_k; } } } (black_ans % 1_000_000_007) as i32 } }
```