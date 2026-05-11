# Maximum Number of Non-Overlapping Subarrays With Sum Equals Target

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Prefix Sum

---

## Problem

<p>Given an array <code>nums</code> and an integer <code>target</code>, return <em>the maximum number of <strong>non-empty</strong> <strong>non-overlapping</strong> subarrays such that the sum of values in each subarray is equal to</em> <code>target</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1,1,1], target = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 non-overlapping subarrays [<strong>1,1</strong>,1,<strong>1,1</strong>] with sum equals to target(2).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [-1,3,5,1,4,2,-9], target = 6
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 3 subarrays with sum equal to 6.
([5,1], [4,2], [3,5,1,4,2,-9]) but only the first 2 are non-overlapping.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= target &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Keep track of prefix sums to quickly look up what subarray that sums "target" can be formed at each step of scanning the input array.
2. It can be proved that greedily forming valid subarrays as soon as one is found is optimal.

## Solution

```rust

```