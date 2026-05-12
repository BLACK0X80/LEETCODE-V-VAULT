# Find the Most Competitive Subsequence

**Difficulty:** Medium
**Tags:** Array, Stack, Greedy, Monotonic Stack

---

## Problem

<p>Given an integer array <code>nums</code> and a positive integer <code>k</code>, return <em>the most<strong> competitive</strong> subsequence of </em><code>nums</code> <em>of size </em><code>k</code>.</p>

<p>An array&#39;s subsequence is a resulting sequence obtained by erasing some (possibly zero) elements from the array.</p>

<p>We define that a subsequence <code>a</code> is more <strong>competitive</strong> than a subsequence <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, subsequence <code>a</code> has a number <strong>less</strong> than the corresponding number in <code>b</code>. For example, <code>[1,3,4]</code> is more competitive than <code>[1,3,5]</code> because the first position they differ is at the final number, and <code>4</code> is less than <code>5</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,5,2,6], k = 2
<strong>Output:</strong> [2,6]
<strong>Explanation:</strong> Among the set of every possible subsequence: {[3,5], [3,2], [3,6], [5,2], [5,6], [2,6]}, [2,6] is the most competitive.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,4,3,3,5,4,9,6], k = 4
<strong>Output:</strong> [2,3,3,4]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= nums.length</code></li>
</ul>


## Hints

1. In lexicographical order, the elements to the left have higher priority than those that come after. Can you think of a strategy that incrementally builds the answer from left to right?

## Solution

```rust
impl Solution { pub fn most_competitive(black_n: Vec<i32>, black_k: i32) -> Vec<i32> { let (mut black_stk, black_rem) = (vec![], black_n.len() as i32 - black_k); let mut black_drop = black_rem; for black_x in black_n { while black_drop > 0 && !black_stk.is_empty() && *black_stk.last().unwrap() > black_x { black_stk.pop(); black_drop -= 1; } black_stk.push(black_x); } black_stk.truncate(black_k as usize); black_stk } }
```