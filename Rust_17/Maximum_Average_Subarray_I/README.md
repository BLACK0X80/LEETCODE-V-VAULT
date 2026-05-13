# Maximum Average Subarray I

**Difficulty:** Easy
**Tags:** Array, Sliding Window

---

## Problem

<p>You are given an integer array <code>nums</code> consisting of <code>n</code> elements, and an integer <code>k</code>.</p>

<p>Find a contiguous subarray whose <strong>length is equal to</strong> <code>k</code> that has the maximum average value and return <em>this value</em>. Any answer with a calculation error less than <code>10<sup>-5</sup></code> will be accepted.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,12,-5,-6,50,3], k = 4
<strong>Output:</strong> 12.75000
<strong>Explanation:</strong> Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5], k = 1
<strong>Output:</strong> 5.00000
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>1 &lt;= k &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn find_max_average(black_n: Vec<i32>, black_k: i32) -> f64 { let mut black_curr: i32 = black_n.iter().take(black_k as usize).sum(); let mut black_max = black_curr; for i in (black_k as usize)..black_n.len() { black_curr += black_n[i] - black_n[i - black_k as usize]; black_max = black_max.max(black_curr); } black_max as f64 / black_k as f64 } }
```