# Maximum Size of a Set After Removals

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code> of even length <code>n</code>.</p>

<p>You must remove <code>n / 2</code> elements from <code>nums1</code> and <code>n / 2</code> elements from <code>nums2</code>. After the removals, you insert the remaining elements of <code>nums1</code> and <code>nums2</code> into a set <code>s</code>.</p>

<p>Return <em>the <strong>maximum</strong> possible size of the set</em> <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,1,2], nums2 = [1,1,1,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We remove two occurences of 1 from nums1 and nums2. After the removals, the arrays become equal to nums1 = [2,2] and nums2 = [1,1]. Therefore, s = {1,2}.
It can be shown that 2 is the maximum possible size of the set s after the removals.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3,4,5,6], nums2 = [2,3,2,3,2,3]
<strong>Output:</strong> 5
<strong>Explanation:</strong> We remove 2, 3, and 6 from nums1, as well as 2 and two occurrences of 3 from nums2. After the removals, the arrays become equal to nums1 = [1,4,5] and nums2 = [2,3,2]. Therefore, s = {1,2,3,4,5}.
It can be shown that 5 is the maximum possible size of the set s after the removals.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,1,2,2,3,3], nums2 = [4,4,5,5,6,6]
<strong>Output:</strong> 6
<strong>Explanation:</strong> We remove 1, 2, and 3 from nums1, as well as 4, 5, and 6 from nums2. After the removals, the arrays become equal to nums1 = [1,2,3] and nums2 = [4,5,6]. Therefore, s = {1,2,3,4,5,6}.
It can be shown that 6 is the maximum possible size of the set s after the removals.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums1.length == nums2.length</code></li>
	<li><code>1 &lt;= n &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>n</code> is even.</li>
	<li><code>1 &lt;= nums1[i], nums2[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Removing <code>n / 2</code> elements from each array is the same as keeping <code>n / 2<code> elements in each array.
2. Think of a greedy algorithm.
3. For each array, we will greedily keep the elements that are only in that array. Once we run out of such elements, we will keep the elements that are common to both arrays.

## Solution

```rust
use std::collections::HashSet; impl Solution { pub fn maximum_set_size(black_n1: Vec<i32>, black_n2: Vec<i32>) -> i32 { let black_n = black_n1.len(); let (black_s1, black_s2): (HashSet<_>, HashSet<_>) = (black_n1.into_iter().collect(), black_n2.into_iter().collect()); let black_common = black_s1.intersection(&black_s2).count(); let black_only1 = black_s1.len() - black_common; let black_only2 = black_s2.len() - black_common; let black_take1 = black_only1.min(black_n / 2); let black_take2 = black_only2.min(black_n / 2); (black_take1 + black_take2 + black_common.min(black_n - black_take1 - black_take2)) as i32 } }
```