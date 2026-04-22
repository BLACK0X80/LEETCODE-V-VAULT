# Maximum Gap

**Difficulty:** Medium
**Tags:** Array, Sorting, Bucket Sort, Radix Sort

---

## Problem

<p>Given an integer array <code>nums</code>, return <em>the maximum difference between two successive elements in its sorted form</em>. If the array contains less than two elements, return <code>0</code>.</p>

<p>You must write an algorithm that runs in linear time and uses linear extra space.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,6,9,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has the maximum difference 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [10]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array contains less than 2 elements, therefore return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return 0; }

        let mut nums: Vec<u32> = nums.iter().map(|&x| x as u32).collect();
        let max = *nums.iter().max().unwrap();
        let mut exp = 1u32;

        while exp <= max {
            let mut buckets = vec![vec![]; 10];
            for &x in &nums {
                buckets[((x / exp) % 10) as usize].push(x);
            }
            nums = buckets.into_iter().flatten().collect();
            exp *= 10;
        }

        nums.windows(2).map(|w| (w[1] - w[0]) as i32).max().unwrap_or(0)
    }
}
```