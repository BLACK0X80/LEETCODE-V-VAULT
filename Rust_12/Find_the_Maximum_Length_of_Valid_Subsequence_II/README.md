# Find the Maximum Length of Valid Subsequence II

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

You are given an integer array <code>nums</code> and a <strong>positive</strong> integer <code>k</code>.
<p>A <span data-keyword="subsequence-array">subsequence</span> <code>sub</code> of <code>nums</code> with length <code>x</code> is called <strong>valid</strong> if it satisfies:</p>

<ul>
	<li><code>(sub[0] + sub[1]) % k == (sub[1] + sub[2]) % k == ... == (sub[x - 2] + sub[x - 1]) % k.</code></li>
</ul>
Return the length of the <strong>longest</strong> <strong>valid</strong> subsequence of <code>nums</code>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4,5], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>The longest valid subsequence is <code>[1, 2, 3, 4, 5]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,4,2,3,1,4], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The longest valid subsequence is <code>[1, 4, 1, 4]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>3</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>7</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>3</sup></code></li>
</ul>


## Hints

1. Fix the value of <code>(subs[0] + subs[1]) % k</code> from the <code>k</code> possible values. Let it be <code>val</code>.
2. Let <code>dp[i]</code> store the maximum length of a subsequence with its last element <code>x</code> such that <code>x % k == i</code>.
3. Answer for a subsequence ending at index <code>y</code> is <code>dp[(k + val - (y % k)) % k] + 1</code>.

## Solution

```rust
impl Solution { pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 { let k = k as usize; let mut black_dp = vec![vec![0; k]; k]; let mut black_ans = 0; for &x in &nums { let black_rem = (x as usize) % k; for prev in 0..k { black_dp[black_rem][prev] = black_dp[prev][black_rem] + 1; black_ans = black_ans.max(black_dp[black_rem][prev]); } } black_ans } }
```