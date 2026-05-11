# Largest Number

**Difficulty:** Medium
**Tags:** Array, String, Greedy, Sorting

---

## Problem

<p>Given a list of non-negative integers <code>nums</code>, arrange them such that they form the largest number and return it.</p>

<p>Since the result may be very large, so you need to return a string instead of an integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,2]
<strong>Output:</strong> &quot;210&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,30,34,5,9]
<strong>Output:</strong> &quot;9534330&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 100</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut black_v: Vec<String> = nums.into_iter().map(|black_x| black_x.to_string()).collect();
        black_v.sort_unstable_by(|black_a, black_b| (black_b.clone() + black_a).cmp(&(black_a.clone() + black_b)));
        if black_v[0] == "0" { return "0".to_string(); }
        black_v.concat()
    }
}
```