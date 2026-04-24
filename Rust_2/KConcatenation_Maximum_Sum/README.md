# K-Concatenation Maximum Sum

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>Given an integer array <code>arr</code> and an integer <code>k</code>, modify the array by repeating it <code>k</code> times.</p>

<p>For example, if <code>arr = [1, 2]</code> and <code>k = 3 </code>then the modified array will be <code>[1, 2, 1, 2, 1, 2]</code>.</p>

<p>Return the maximum sub-array sum in the modified array. Note that the length of the sub-array can be <code>0</code> and its sum in that case is <code>0</code>.</p>

<p>As the answer can be very large, return the answer <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2], k = 3
<strong>Output:</strong> 9
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,-2,1], k = 5
<strong>Output:</strong> 2
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [-1,-2], k = 7
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= arr[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. How to solve the problem for k=1 ?
2. Use Kadane's algorithm for k=1.
3. What are the possible cases for the answer ?
4. The answer is the maximum between, the answer for k=1, the sum of the whole array multiplied by k, or the maximum suffix sum plus the maximum prefix sum plus (k-2) multiplied by the whole array sum for k > 1.

## Solution

```rust
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let kadane = |a: &[i32]| -> i64 {
            let (mut max, mut cur) = (0i64, 0i64);
            for &x in a { cur = (cur + x as i64).max(0); max = max.max(cur); }
            max
        };
        let total: i64 = arr.iter().map(|&x| x as i64).sum();
        let one = kadane(&arr);
        if k == 1 { return (one % MOD) as i32; }
        let two = kadane(&arr.iter().chain(arr.iter()).cloned().collect::<Vec<_>>());
        let ans = if total > 0 { (two + total * (k as i64 - 2)) % MOD } else { two % MOD };
        ans as i32
    }
}
```