# Number of Longest Increasing Subsequence

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Binary Indexed Tree, Segment Tree

---

## Problem

<p>Given an integer array&nbsp;<code>nums</code>, return <em>the number of longest increasing subsequences.</em></p>

<p><strong>Notice</strong> that the sequence has to be <strong>strictly</strong> increasing.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,5,4,7]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2,2,2,2]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The length of the longest increasing subsequence is 1, and there are 5 increasing subsequences of length 1, so output 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2000</code></li>
	<li><code>-10<sup>6</sup> &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li>The answer is guaranteed to fit inside a 32-bit integer.</li>
</ul>



## Solution

```rust
impl Solution { pub fn find_number_of_lis(black_n: Vec<i32>) -> i32 { let n = black_n.len(); let (mut black_len, mut black_cnt) = (vec![1; n], vec![1; n]); let (mut max_l, mut res) = (1, 0); for i in 0..n { for j in 0..i { if black_n[i] > black_n[j] { if black_len[j] + 1 > black_len[i] { black_len[i] = black_len[j] + 1; black_cnt[i] = black_cnt[j]; } else if black_len[j] + 1 == black_len[i] { black_cnt[i] += black_cnt[j]; } } } if black_len[i] > max_l { max_l = black_len[i]; res = black_cnt[i]; } else if black_len[i] == max_l { res += black_cnt[i]; } } res } }
```