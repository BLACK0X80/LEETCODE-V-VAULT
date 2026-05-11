# Maximum Number of Ways to Partition an Array

**Difficulty:** Hard
**Tags:** Array, Hash Table, Counting, Enumeration, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> of length <code>n</code>. The number of ways to <strong>partition</strong> <code>nums</code> is the number of <code>pivot</code> indices that satisfy both conditions:</p>

<ul>
	<li><code>1 &lt;= pivot &lt; n</code></li>
	<li><code>nums[0] + nums[1] + ... + nums[pivot - 1] == nums[pivot] + nums[pivot + 1] + ... + nums[n - 1]</code></li>
</ul>

<p>You are also given an integer <code>k</code>. You can choose to change the value of <strong>one</strong> element of <code>nums</code> to <code>k</code>, or to leave the array <strong>unchanged</strong>.</p>

<p>Return <em>the <strong>maximum</strong> possible number of ways to <strong>partition</strong> </em><code>nums</code><em> to satisfy both conditions after changing <strong>at most</strong> one element</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,-1,2], k = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> One optimal approach is to change nums[0] to k. The array becomes [<strong><u>3</u></strong>,-1,2].
There is one way to partition the array:
- For pivot = 2, we have the partition [3,-1 | 2]: 3 + -1 == 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,0,0], k = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> The optimal approach is to leave the array unchanged.
There are two ways to partition the array:
- For pivot = 1, we have the partition [0 | 0,0]: 0 == 0 + 0.
- For pivot = 2, we have the partition [0,0 | 0]: 0 + 0 == 0.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [22,4,-25,-20,-15,15,-16,7,19,-10,0,-13,-14], k = -33
<strong>Output:</strong> 4
<strong>Explanation:</strong> One optimal approach is to change nums[2] to k. The array becomes [22,4,<u><strong>-33</strong></u>,-20,-15,15,-16,7,19,-10,0,-13,-14].
There are four ways to partition the array.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= k, nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. A pivot point splits the array into equal prefix and suffix. If no change is made to the array, the goal is to find the number of pivot p such that prefix[p-1] == suffix[p].
2. Consider how prefix and suffix will change when we change a number nums[i] to k.
3. When sweeping through each element, can you find the total number of pivots where the difference of prefix and suffix happens to equal to the changes of k-nums[i].

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn ways_to_partition(black1: Vec<i32>, black2: i32) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![0i64; black3];
        black4[0] = black1[0] as i64;
        for black5 in 1..black3 { black4[black5] = black4[black5 - 1] + black1[black5] as i64; }
        
        let black6 = black4[black3 - 1];
        let mut black7 = HashMap::new();
        let mut black8 = HashMap::new();
        
        for black9 in 0..black3 - 1 {
            *black8.entry(black4[black9]).or_insert(0) += 1;
        }
        
        let mut black10 = 0;
        if black6 % 2 == 0 {
            black10 = *black8.get(&(black6 / 2)).unwrap_or(&0);
        }
        
        for black11 in 0..black3 {
            let black12 = black2 as i64 - black1[black11] as i64;
            let black13 = black6 + black12;
            let mut black14 = 0;
            
            if black13 % 2 == 0 {
                let black15 = black13 / 2;
                black14 += *black7.get(&black15).unwrap_or(&0);
                black14 += *black8.get(&(black15 - black12)).unwrap_or(&0);
            }
            
            black10 = black10.max(black14);
            
            if black11 < black3 - 1 {
                let black16 = black4[black11];
                *black8.entry(black16).or_insert(0) -= 1;
                *black7.entry(black16).or_insert(0) += 1;
            }
        }
        
        black10 as i32
    }
}
```