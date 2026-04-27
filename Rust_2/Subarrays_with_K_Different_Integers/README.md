# Subarrays with K Different Integers

**Difficulty:** Hard
**Tags:** Array, Hash Table, Sliding Window, Counting

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, return <em>the number of <strong>good subarrays</strong> of </em><code>nums</code>.</p>

<p>A <strong>good array</strong> is an array where the number of different integers in that array is exactly <code>k</code>.</p>

<ul>
	<li>For example, <code>[1,2,3,1,2]</code> has <code>3</code> different integers: <code>1</code>, <code>2</code>, and <code>3</code>.</li>
</ul>

<p>A <strong>subarray</strong> is a <strong>contiguous</strong> part of an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,2,3], k = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,3,4], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i], k &lt;= nums.length</code></li>
</ul>


## Hints

1. Try generating all possible subarrays and check for the number of unique integers. Increment the count accordingly.
2. How about using a map to store the count of integers?
3. Think about the Sliding Window and 2-pointer approach.

## Solution

```rust
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let at_most = |k: i32| -> i32 {
            let mut cnt = std::collections::HashMap::new();
            let (mut l, mut res) = (0usize, 0i32);
            for r in 0..nums.len() {
                *cnt.entry(nums[r]).or_insert(0) += 1;
                while cnt.len() as i32 > k {
                    let e = cnt.get_mut(&nums[l]).unwrap();
                    *e -= 1;
                    if *e == 0 { cnt.remove(&nums[l]); }
                    l += 1;
                }
                res += (r - l + 1) as i32;
            }
            res
        };
        at_most(k) - at_most(k - 1)
    }
}
```