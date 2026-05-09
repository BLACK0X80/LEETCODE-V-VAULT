# Maximum Erasure Value

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sliding Window

---

## Problem

<p>You are given an array of positive integers <code>nums</code> and want to erase a subarray containing&nbsp;<strong>unique elements</strong>. The <strong>score</strong> you get by erasing the subarray is equal to the <strong>sum</strong> of its elements.</p>

<p>Return <em>the <strong>maximum score</strong> you can get by erasing <strong>exactly one</strong> subarray.</em></p>

<p>An array <code>b</code> is called to be a <span class="tex-font-style-it">subarray</span> of <code>a</code> if it forms a contiguous subsequence of <code>a</code>, that is, if it is equal to <code>a[l],a[l+1],...,a[r]</code> for some <code>(l,r)</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,2,4,5,6]
<strong>Output:</strong> 17
<strong>Explanation:</strong> The optimal subarray here is [2,4,5,6].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,2,1,2,5,2,1,2,5]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The optimal subarray here is [5,2,1] or [1,2,5].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. The main point here is for the subarray to contain unique elements for each index. Only the first subarrays starting from that index have unique elements.
2. This can be solved using the two pointers technique

## Solution

```rust
impl Solution { pub fn maximum_unique_subarray(black_n: Vec<i32>) -> i32 { let (mut black_v, mut black_l, mut black_s, mut black_res) = ([false; 10001], 0, 0, 0); for black_r in 0..black_n.len() { let black_val = black_n[black_r] as usize; while black_v[black_val] { let black_left_val = black_n[black_l] as usize; black_v[black_left_val] = false; black_s -= black_n[black_l]; black_l += 1; } black_v[black_val] = true; black_s += black_n[black_r]; if black_s > black_res { black_res = black_s; } } black_res } }
```