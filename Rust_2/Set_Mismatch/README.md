# Set Mismatch

**Difficulty:** Easy
**Tags:** Array, Hash Table, Bit Manipulation, Sorting

---

## Problem

<p>You have a set of integers <code>s</code>, which originally contains all the numbers from <code>1</code> to <code>n</code>. Unfortunately, due to some error, one of the numbers in <code>s</code> got duplicated to another number in the set, which results in <strong>repetition of one</strong> number and <strong>loss of another</strong> number.</p>

<p>You are given an integer array <code>nums</code> representing the data status of this set after the error.</p>

<p>Find the number that occurs twice and the number that is missing and return <em>them in the form of an array</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [1,2,2,4]
<strong>Output:</strong> [2,3]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [1,1]
<strong>Output:</strong> [1,2]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = vec![0; nums.len() + 1];
        for n in &nums { freq[*n as usize] += 1; }
        let (mut dup, mut miss) = (0, 0);
        for i in 1..=nums.len() { if freq[i] == 2 { dup = i as i32; } else if freq[i] == 0 { miss = i as i32; } }
        vec![dup, miss]
    }
}
```