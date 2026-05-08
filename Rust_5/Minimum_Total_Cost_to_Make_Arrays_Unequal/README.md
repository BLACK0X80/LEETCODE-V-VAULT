# Minimum Total Cost to Make Arrays Unequal

**Difficulty:** Hard
**Tags:** Array, Hash Table, Greedy, Counting

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code>, of equal length <code>n</code>.</p>

<p>In one operation, you can swap the values of any two indices of <code>nums1</code>. The <strong>cost</strong> of this operation is the <strong>sum</strong> of the indices.</p>

<p>Find the <strong>minimum</strong> total cost of performing the given operation <strong>any</strong> number of times such that <code>nums1[i] != nums2[i]</code> for all <code>0 &lt;= i &lt;= n - 1</code> after performing all the operations.</p>

<p>Return <em>the <strong>minimum total cost</strong> such that </em><code>nums1</code> and <code>nums2</code><em> satisfy the above condition</em>. In case it is not possible, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3,4,5], nums2 = [1,2,3,4,5]
<strong>Output:</strong> 10
<strong>Explanation:</strong> 
One of the ways we can perform the operations is:
- Swap values at indices 0 and 3, incurring cost = 0 + 3 = 3. Now, nums1 = [4,2,3,1,5]
- Swap values at indices 1 and 2, incurring cost = 1 + 2 = 3. Now, nums1 = [4,3,2,1,5].
- Swap values at indices 0 and 4, incurring cost = 0 + 4 = 4. Now, nums1 =[5,3,2,1,4].
We can see that for each index i, nums1[i] != nums2[i]. The cost required here is 10.
Note that there are other ways to swap values, but it can be proven that it is not possible to obtain a cost less than 10.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [2,2,2,1,3], nums2 = [1,2,2,3,3]
<strong>Output:</strong> 10
<strong>Explanation:</strong> 
One of the ways we can perform the operations is:
- Swap values at indices 2 and 3, incurring cost = 2 + 3 = 5. Now, nums1 = [2,2,1,2,3].
- Swap values at indices 1 and 4, incurring cost = 1 + 4 = 5. Now, nums1 = [2,3,1,2,2].
The total cost needed here is 10, which is the minimum possible.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,2], nums2 = [1,2,2]
<strong>Output:</strong> -1
<strong>Explanation:</strong> 
It can be shown that it is not possible to satisfy the given conditions irrespective of the number of operations we perform.
Hence, we return -1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums1.length == nums2.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums1[i], nums2[i] &lt;= n</code></li>
</ul>


## Hints

1. How can we check which indices of <code>nums1</code> will be considered for swapping? How to minimize the number of such operations?
2. It can be seen that greedily swapping values of indices where <code>nums1[i] == nums2[i]</code> is the most optimal choice. How many values cannot be swapped this way?
3. Find which indices we will swap these remaining values with, and if there are enough such indices.

## Solution

```rust
impl Solution {
    pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut black = (0i64, 0, 0, 0, vec![0; nums1.len() + 1]);
        for black_i in 0..nums1.len() {
            if nums1[black_i] == nums2[black_i] {
                let black_v = nums1[black_i] as usize;
                black.4[black_v] += 1;
                if black.4[black_v] > black.2 {
                    black.2 = black.4[black_v];
                    black.3 = black_v as i32;
                }
                black.0 += black_i as i64;
                black.1 += 1;
            }
        }
        let mut black_need = 2 * black.2 - black.1;
        if black_need > 0 {
            for black_i in 0..nums1.len() {
                if nums1[black_i] != nums2[black_i] && nums1[black_i] != black.3 && nums2[black_i] != black.3 {
                    black.0 += black_i as i64;
                    black.1 += 1;
                    black_need -= 1;
                    if black_need == 0 { break; }
                }
            }
        }
        if 2 * black.2 > black.1 { -1 } else { black.0 }
    }
}
```