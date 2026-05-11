# Majority Element II

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sorting, Counting

---

## Problem

<p>Given an integer array of size <code>n</code>, find all elements that appear more than <code>&lfloor; n/3 &rfloor;</code> times.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,3]
<strong>Output:</strong> [3]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1]
<strong>Output:</strong> [1]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2]
<strong>Output:</strong> [1,2]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> Could you solve the problem in linear time and in <code>O(1)</code> space?</p>


## Hints

1. Think about the possible number of elements that can appear more than ⌊ n/3 ⌋ times in the array.
2. It can be at most two. Why?
3. Consider using Boyer-Moore Voting Algorithm, which is efficient for finding elements that appear more than a certain threshold.

## Solution

```rust
impl Solution { pub fn majority_element(black_n: Vec<i32>) -> Vec<i32> { let (mut black_c1, mut black_c2, mut black_v1, mut black_v2) = (0, 0, 0, 1); for &black_x in &black_n { if black_x == black_v1 { black_c1 += 1; } else if black_x == black_v2 { black_c2 += 1; } else if black_c1 == 0 { black_v1 = black_x; black_c1 = 1; } else if black_c2 == 0 { black_v2 = black_x; black_c2 = 1; } else { black_c1 -= 1; black_c2 -= 1; } } [black_v1, black_v2].iter().filter(|&&black_v| black_n.iter().filter(|&&black_x| black_x == black_v).count() > black_n.len() / 3).cloned().collect::<std::collections::HashSet<_>>().into_iter().collect() } }
```