# Subarray Sum Equals K

**Difficulty:** Medium
**Tags:** Array, Hash Table, Prefix Sum

---

## Problem

<p>Given an array of integers <code>nums</code> and an integer <code>k</code>, return <em>the total number of subarrays whose sum equals to</em> <code>k</code>.</p>

<p>A subarray is a contiguous <strong>non-empty</strong> sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [1,1,1], k = 2
<strong>Output:</strong> 2
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [1,2,3], k = 3
<strong>Output:</strong> 2
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>-1000 &lt;= nums[i] &lt;= 1000</code></li>
	<li><code>-10<sup>7</sup> &lt;= k &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. Will Brute force work here? Try to optimize it.
2. Can we optimize it by using some extra space?
3. What about storing sum frequencies in a hash table? Will it be useful?
4. sum(i,j)=sum(0,j)-sum(0,i), where sum(i,j) represents the sum of all the elements from index i to j-1.

Can we use this property to optimize it.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let mut black_map = HashMap::new();
        black_map.insert(0, 1);
        let mut black_count = 0;
        let mut black_sum = 0;

        for black_n in black_nums {
            black_sum += black_n;
            if let Some(&black_prev) = black_map.get(&(black_sum - black_k)) {
                black_count += black_prev;
            }
            let bravexuneth = black_map.entry(black_sum).or_insert(0);
            *bravexuneth += 1;
        }

        black_count
    }
}
```