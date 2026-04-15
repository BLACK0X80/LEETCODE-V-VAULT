# Contains Duplicate III

**Difficulty:** Hard
**Tags:** Array, Sliding Window, Sorting, Bucket Sort, Ordered Set

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers <code>indexDiff</code> and <code>valueDiff</code>.</p>

<p>Find a pair of indices <code>(i, j)</code> such that:</p>

<ul>
	<li><code>i != j</code>,</li>
	<li><code>abs(i - j) &lt;= indexDiff</code>.</li>
	<li><code>abs(nums[i] - nums[j]) &lt;= valueDiff</code>, and</li>
</ul>

<p>Return <code>true</code><em> if such pair exists or </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,1], indexDiff = 3, valueDiff = 0
<strong>Output:</strong> true
<strong>Explanation:</strong> We can choose (i, j) = (0, 3).
We satisfy the three conditions:
i != j --&gt; 0 != 3
abs(i - j) &lt;= indexDiff --&gt; abs(0 - 3) &lt;= 3
abs(nums[i] - nums[j]) &lt;= valueDiff --&gt; abs(1 - 1) &lt;= 0
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,5,9,1,5,9], indexDiff = 2, valueDiff = 3
<strong>Output:</strong> false
<strong>Explanation:</strong> After trying all the possible pairs (i, j), we cannot satisfy the three conditions, so we return false.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= indexDiff &lt;= nums.length</code></li>
	<li><code>0 &lt;= valueDiff &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Time complexity O(n logk)  - This will give an indication that sorting is involved for k elements.
2. Use already existing state to evaluate next state  -  Like, a set of k sorted numbers are only needed to be tracked. When we are processing the next number in array, then we can utilize the existing sorted state and it is not necessary to sort next overlapping set of k numbers again.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let k = index_diff as usize;
        let t = value_diff as i64;
        let w = t + 1;
        let mut buckets: HashMap<i64, i64> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let n = num as i64;
            let id = if n >= 0 { n / w } else { (n + 1) / w - 1 };

            if buckets.contains_key(&id) { return true; }
            if buckets.get(&(id - 1)).map_or(false, |&v| n - v <= t) { return true; }
            if buckets.get(&(id + 1)).map_or(false, |&v| v - n <= t) { return true; }

            buckets.insert(id, n);
            if i >= k { 
                let old = nums[i - k] as i64;
                let old_id = if old >= 0 { old / w } else { (old + 1) / w - 1 };
                buckets.remove(&old_id);
            }
        }

        false
    }
}
```