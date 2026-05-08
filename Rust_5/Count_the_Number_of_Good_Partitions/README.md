# Count the Number of Good Partitions

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Combinatorics

---

## Problem

<p>You are given a <strong>0-indexed</strong> array <code>nums</code> consisting of <strong>positive</strong> integers.</p>

<p>A partition of an array into one or more <strong>contiguous</strong> subarrays is called <strong>good</strong> if no two subarrays contain the same number.</p>

<p>Return <em>the <strong>total number</strong> of good partitions of </em><code>nums</code>.</p>

<p>Since the answer may be large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The 8 possible good partitions are: ([1], [2], [3], [4]), ([1], [2], [3,4]), ([1], [2,3], [4]), ([1], [2,3,4]), ([1,2], [3], [4]), ([1,2], [3,4]), ([1,2,3], [4]), and ([1,2,3,4]).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only possible good partition is: ([1,1,1,1]).
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The 2 possible good partitions are: ([1,2,1], [3]) and ([1,2,1,3]).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. If a segment contains a value, it must contain all occurrences of the same value.
2. Partition the array into segments making each one as short as possible. This can be achieved by two-pointers or using a Set.
3. If we have <code>m</code> segments, we can arbitrarily group the neighboring segments. How many ways are there to group these <code>m</code> segments?

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_good_partitions(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let mut black_last = HashMap::new();
        for i in 0..black_n { black_last.insert(black_nums[i], i); }

        let (mut black_res, mut black_j, mut black_parts) = (1i64, 0, 0);
        let mut i = 0;
        while i < black_n {
            if i > black_j { black_parts += 1; }
            black_j = black_j.max(black_last[&black_nums[i]]);
            i += 1;
        }

        for _ in 0..black_parts { black_res = (black_res * 2) % 1_000_000_007; }
        black_res as i32
    }
}
```