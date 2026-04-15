# Minimum Difference in Sums After Removal of Elements

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Heap (Priority Queue)

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> consisting of <code>3 * n</code> elements.</p>

<p>You are allowed to remove any <strong>subsequence</strong> of elements of size <strong>exactly</strong> <code>n</code> from <code>nums</code>. The remaining <code>2 * n</code> elements will be divided into two <strong>equal</strong> parts:</p>

<ul>
	<li>The first <code>n</code> elements belonging to the first part and their sum is <code>sum<sub>first</sub></code>.</li>
	<li>The next <code>n</code> elements belonging to the second part and their sum is <code>sum<sub>second</sub></code>.</li>
</ul>

<p>The <strong>difference in sums</strong> of the two parts is denoted as <code>sum<sub>first</sub> - sum<sub>second</sub></code>.</p>

<ul>
	<li>For example, if <code>sum<sub>first</sub> = 3</code> and <code>sum<sub>second</sub> = 2</code>, their difference is <code>1</code>.</li>
	<li>Similarly, if <code>sum<sub>first</sub> = 2</code> and <code>sum<sub>second</sub> = 3</code>, their difference is <code>-1</code>.</li>
</ul>

<p>Return <em>the <strong>minimum difference</strong> possible between the sums of the two parts after the removal of </em><code>n</code><em> elements</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,1,2]
<strong>Output:</strong> -1
<strong>Explanation:</strong> Here, nums has 3 elements, so n = 1. 
Thus we have to remove 1 element from nums and divide the array into two equal parts.
- If we remove nums[0] = 3, the array will be [1,2]. The difference in sums of the two parts will be 1 - 2 = -1.
- If we remove nums[1] = 1, the array will be [3,2]. The difference in sums of the two parts will be 3 - 2 = 1.
- If we remove nums[2] = 2, the array will be [3,1]. The difference in sums of the two parts will be 3 - 1 = 2.
The minimum difference between sums of the two parts is min(-1,1,2) = -1. 
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [7,9,5,8,1,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Here n = 2. So we must remove 2 elements and divide the remaining array into two parts containing two elements each.
If we remove nums[2] = 5 and nums[3] = 8, the resultant array will be [7,9,1,3]. The difference in sums will be (7+9) - (1+3) = 12.
To obtain the minimum difference, we should remove nums[1] = 9 and nums[4] = 1. The resultant array becomes [7,5,8,3]. The difference in sums of the two parts is (7+5) - (8+3) = 1.
It can be shown that it is not possible to obtain a difference smaller than 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>nums.length == 3 * n</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. The lowest possible difference can be obtained when the sum of the first n elements in the resultant array is minimum, and the sum of the next n elements is maximum.
2. For every index i, think about how you can find the minimum possible sum of n elements with indices lesser or equal to i, if possible.
3. Similarly, for every index i, try to find the maximum possible sum of n elements with indices greater or equal to i, if possible.
4. Now for all indices, check if we can consider it as the partitioning index and hence find the answer.

## Solution

```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(black1: Vec<i32>) -> i64 {
        let black2 = black1.len();
        let black3 = black2 / 3;
        let mut black4 = vec![0i64; black2 + 1];
        let mut black5 = vec![0i64; black2 + 1];
        let mut black6 = BinaryHeap::new();
        let mut black7 = 0i64;

        for i in 0..black3 {
            black7 += black1[i] as i64;
            black6.push(black1[i]);
        }
        black4[black3] = black7;
        for i in black3..(2 * black3) {
            black7 += black1[i] as i64;
            black6.push(black1[i]);
            black7 -= black6.pop().unwrap() as i64;
            black4[i + 1] = black7;
        }

        let mut black8 = BinaryHeap::new();
        let mut black9 = 0i64;
        for i in (2 * black3..black2).rev() {
            black9 += black1[i] as i64;
            black8.push(std::cmp::Reverse(black1[i]));
        }
        black5[2 * black3] = black9;
        for i in (black3..2 * black3).rev() {
            black9 += black1[i] as i64;
            black8.push(std::cmp::Reverse(black1[i]));
            black9 -= black8.pop().unwrap().0 as i64;
            black5[i] = black9;
        }

        let mut black10 = i64::MAX;
        for i in black3..=(2 * black3) {
            black10 = black10.min(black4[i] - black5[i]);
        }
        black10
    }
}
```