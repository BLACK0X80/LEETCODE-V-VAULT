# Find the Difference of Two Arrays

**Difficulty:** Easy
**Tags:** Array, Hash Table

---

## Problem

<p>Given two <strong>0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code>, return <em>a list</em> <code>answer</code> <em>of size</em> <code>2</code> <em>where:</em></p>

<ul>
	<li><code>answer[0]</code> <em>is a list of all <strong>distinct</strong> integers in</em> <code>nums1</code> <em>which are <strong>not</strong> present in</em> <code>nums2</code><em>.</em></li>
	<li><code>answer[1]</code> <em>is a list of all <strong>distinct</strong> integers in</em> <code>nums2</code> <em>which are <strong>not</strong> present in</em> <code>nums1</code>.</li>
</ul>

<p><strong>Note</strong> that the integers in the lists may be returned in <strong>any</strong> order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3], nums2 = [2,4,6]
<strong>Output:</strong> [[1,3],[4,6]]
<strong>Explanation:
</strong>For nums1, nums1[1] = 2 is present at index 0 of nums2, whereas nums1[0] = 1 and nums1[2] = 3 are not present in nums2. Therefore, answer[0] = [1,3].
For nums2, nums2[0] = 2 is present at index 1 of nums1, whereas nums2[1] = 4 and nums2[2] = 6 are not present in nums1. Therefore, answer[1] = [4,6].</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3,3], nums2 = [1,1,2,2]
<strong>Output:</strong> [[3],[]]
<strong>Explanation:
</strong>For nums1, nums1[2] and nums1[3] are not present in nums2. Since nums1[2] == nums1[3], their value is only included once and answer[0] = [3].
Every integer in nums2 is present in nums1. Therefore, answer[1] = [].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length, nums2.length &lt;= 1000</code></li>
	<li><code>-1000 &lt;= nums1[i], nums2[i] &lt;= 1000</code></li>
</ul>


## Hints

1. For each integer in nums1, check if it exists in nums2.
2. Do the same for each integer in nums2.

## Solution

```rust
impl Solution { pub fn find_difference(black_1: Vec<i32>, black_2: Vec<i32>) -> Vec<Vec<i32>> { let (mut black_b1, mut black_b2) = ([false; 2001], [false; 2001]); for x in black_1.iter() { black_b1[(x + 1000) as usize] = true; } for x in black_2.iter() { black_b2[(x + 1000) as usize] = true; } vec![black_1.into_iter().filter(|x| { let i = (x + 1000) as usize; if black_b1[i] && !black_b2[i] { black_b1[i] = false; true } else { false } }).collect(), black_2.into_iter().filter(|x| { let i = (x + 1000) as usize; if black_b2[i] && !black_b1[i] { black_b2[i] = false; true } else { false } }).collect()] } }
```