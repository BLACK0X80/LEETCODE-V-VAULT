# Minimize the Maximum Difference of Pairs

**Difficulty:** Medium
**Tags:** Array, Binary Search, Dynamic Programming, Greedy, Sorting

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> and an integer <code>p</code>. Find <code>p</code> pairs of indices of <code>nums</code> such that the <strong>maximum</strong> difference amongst all the pairs is <strong>minimized</strong>. Also, ensure no index appears more than once amongst the <code>p</code> pairs.</p>

<p>Note that for a pair of elements at the index <code>i</code> and <code>j</code>, the difference of this pair is <code>|nums[i] - nums[j]|</code>, where <code>|x|</code> represents the <strong>absolute</strong> <strong>value</strong> of <code>x</code>.</p>

<p>Return <em>the <strong>minimum</strong> <strong>maximum</strong> difference among all </em><code>p</code> <em>pairs.</em> We define the maximum of an empty set to be zero.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,1,2,7,1,3], p = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> The first pair is formed from the indices 1 and 4, and the second pair is formed from the indices 2 and 5. 
The maximum difference is max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) = max(0, 1) = 1. Therefore, we return 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,2,1,2], p = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> Let the indices 1 and 3 form a pair. The difference of that pair is |2 - 2| = 0, which is the minimum we can attain.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= p &lt;= (nums.length)/2</code></li>
</ul>


## Hints

1. To minimize the answer, the array should be sorted first.
2. Can we use binary search here?

## Solution

```rust
impl Solution { pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 { nums.sort(); let (mut black_l, mut black_r) = (0, nums.last().unwrap() - nums[0]); while black_l < black_r { let black_mid = black_l + (black_r - black_l) / 2; let (mut black_cnt, mut i) = (0, 1); while i < nums.len() { if nums[i] - nums[i-1] <= black_mid { black_cnt += 1; i += 2; } else { i += 1; } } if black_cnt >= p { black_r = black_mid; } else { black_l = black_mid + 1; } } black_l } }
```