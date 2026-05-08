# Equal Sum Arrays With Minimum Number of Operations

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Counting

---

## Problem

<p>You are given two arrays of integers <code>nums1</code> and <code><font face="monospace">nums2</font></code>, possibly of different lengths. The values in the arrays are between <code>1</code> and <code>6</code>, inclusive.</p>

<p>In one operation, you can change any integer&#39;s value in <strong>any </strong>of the arrays to <strong>any</strong> value between <code>1</code> and <code>6</code>, inclusive.</p>

<p>Return <em>the minimum number of operations required to make the sum of values in </em><code>nums1</code><em> equal to the sum of values in </em><code>nums2</code><em>.</em> Return <code>-1</code>​​​​​ if it is not possible to make the sum of the two arrays equal.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3,4,5,6], nums2 = [1,1,2,2,2,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums2[0] to 6. nums1 = [1,2,3,4,5,6], nums2 = [<u><strong>6</strong></u>,1,2,2,2,2].
- Change nums1[5] to 1. nums1 = [1,2,3,4,5,<strong><u>1</u></strong>], nums2 = [6,1,2,2,2,2].
- Change nums1[2] to 2. nums1 = [1,2,<strong><u>2</u></strong>,4,5,1], nums2 = [6,1,2,2,2,2].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,1,1,1,1,1,1], nums2 = [6]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no way to decrease the sum of nums1 or to increase the sum of nums2 to make them equal.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [6,6], nums2 = [1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed. 
- Change nums1[0] to 2. nums1 = [<strong><u>2</u></strong>,6], nums2 = [1].
- Change nums1[1] to 2. nums1 = [2,<strong><u>2</u></strong>], nums2 = [1].
- Change nums2[0] to 4. nums1 = [2,2], nums2 = [<strong><u>4</u></strong>].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length, nums2.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums1[i], nums2[i] &lt;= 6</code></li>
</ul>


## Hints

1. Let's note that we want to either decrease the sum of the array with a larger sum or increase the array's sum with the smaller sum.
2. You can maintain the largest increase or decrease you can make in a binary search tree and each time get the maximum one.

## Solution

```rust
impl Solution { pub fn min_operations(mut n1: Vec<i32>, mut n2: Vec<i32>) -> i32 { let (mut s1, mut s2) = (n1.iter().sum::<i32>(), n2.iter().sum::<i32>()); if s1 > s2 { std::mem::swap(&mut n1, &mut n2); std::mem::swap(&mut s1, &mut s2); } let (mut black_diff, mut black_res, mut black_gains) = (s2 - s1, 0, vec![0; 6]); n1.iter().for_each(|&x| black_gains[(6 - x) as usize] += 1); n2.iter().for_each(|&x| black_gains[(x - 1) as usize] += 1); for i in (1..6).rev() { let black_take = (black_gains[i]).min((black_diff + i as i32 - 1) / i as i32); black_res += black_take; black_diff -= black_take * i as i32; if black_diff <= 0 { break; } } if black_diff > 0 { -1 } else { black_res } } }
```