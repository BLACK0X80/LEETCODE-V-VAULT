# Number of Subarrays With AND Value of K

**Difficulty:** Hard
**Tags:** Array, Binary Search, Bit Manipulation, Segment Tree

---

## Problem

<p>Given an array of integers <code>nums</code> and an integer <code>k</code>, return the number of <span data-keyword="subarray-nonempty">subarrays</span> of <code>nums</code> where the bitwise <code>AND</code> of the elements of the subarray equals <code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,1], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>All subarrays contain only 1&#39;s.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,2], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>Subarrays having an <code>AND</code> value of 1 are: <code>[<u><strong>1</strong></u>,1,2]</code>, <code>[1,<u><strong>1</strong></u>,2]</code>, <code>[<u><strong>1,1</strong></u>,2]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Subarrays having an <code>AND</code> value of 2 are: <code>[1,<b><u>2</u></b>,3]</code>, <code>[1,<u><strong>2,3</strong></u>]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i], k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Let’s say we want to count the number of pairs <code>(l, r)</code> such that <code>nums[l] & nums[l + 1] & … & nums[r] == k</code>.
2. Fix the left index <code>l</code>.
3. Note that if you increase <code>r</code> for a fixed <code>l</code>, then the AND value of the subarray either decreases or remains unchanged.
4. Therefore, consider using binary search.
5. To calculate the AND value of a subarray, use sparse tables.

## Solution

```rust
impl Solution {
    pub fn count_subarrays(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let (mut black_res, mut black_l1, mut black_l2) = (0i64, 0, 0);
        let (mut black_c1, mut black_c2) = (vec![0; 32], vec![0; 32]);
        for black_r in 0..black_nums.len() {
            for black_i in 0..32 { if (black_nums[black_r] >> black_i) & 1 == 0 { black_c1[black_i] += 1; black_c2[black_i] += 1; } }
            while black_l1 <= black_r && (0..32).fold(u32::MAX, |black_a, black_i| if black_c1[black_i] == 0 { black_a } else { black_a & !(1 << black_i) }) < black_k as u32 {
                for black_i in 0..32 { if (black_nums[black_l1] >> black_i) & 1 == 0 { black_c1[black_i] -= 1; } }
                black_l1 += 1;
            }
            while black_l2 <= black_r && (0..32).fold(u32::MAX, |black_a, black_i| if black_c2[black_i] == 0 { black_a } else { black_a & !(1 << black_i) }) <= black_k as u32 {
                for black_i in 0..32 { if (black_nums[black_l2] >> black_i) & 1 == 0 { black_c2[black_i] -= 1; } }
                black_l2 += 1;
            }
            black_res += (black_l2 - black_l1) as i64;
        }
        black_res
    }
}
```