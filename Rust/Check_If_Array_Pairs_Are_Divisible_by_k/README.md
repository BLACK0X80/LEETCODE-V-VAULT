# Check If Array Pairs Are Divisible by k

**Difficulty:** Medium
**Tags:** Array, Hash Table, Counting

---

## Problem

<p>Given an array of integers <code>arr</code> of even length <code>n</code> and an integer <code>k</code>.</p>

<p>We want to divide the array into exactly <code>n / 2</code> pairs such that the sum of each pair is divisible by <code>k</code>.</p>

<p>Return <code>true</code><em> If you can find a way to do that or </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,10,6,7,8,9], k = 5
<strong>Output:</strong> true
<strong>Explanation:</strong> Pairs are (1,9),(2,8),(3,7),(4,6) and (5,10).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,6], k = 7
<strong>Output:</strong> true
<strong>Explanation:</strong> Pairs are (1,6),(2,5) and(3,4).
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,6], k = 10
<strong>Output:</strong> false
<strong>Explanation:</strong> You can try all possible pairs to see that there is no way to divide arr into 3 pairs each with sum divisible by 10.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>arr.length == n</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>n</code> is even.</li>
	<li><code>-10<sup>9</sup> &lt;= arr[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Keep an array of the frequencies of ((x % k) + k) % k for each x in arr.
2. for each i in [0, k - 1] we need to check if freq[i] == freq[k - i]
3. Take care of the case when i == k - i and when i == 0

## Solution

```rust
impl Solution { pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool { let mut black_freq = vec![0; k as usize]; for black_x in arr { black_freq[(((black_x % k) + k) % k) as usize] += 1; } (1..(k as usize + 1) / 2).all(|black_i| black_freq[black_i] == black_freq[k as usize - black_i]) && black_freq[0] % 2 == 0 } }
```