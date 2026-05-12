# Maximum Alternating Sum of Squares

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given an integer array <code>nums</code>. You may <strong>rearrange the elements</strong> in any order.</p>

<p>The <strong>alternating score</strong> of an array <code>arr</code> is defined as:</p>

<ul>
	<li><code>score = arr[0]<sup>2</sup> - arr[1]<sup>2</sup> + arr[2]<sup>2</sup> - arr[3]<sup>2</sup> + ...</code></li>
</ul>

<p>Return an integer denoting the <strong>maximum possible alternating score</strong> of <code>nums</code> after rearranging its elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>A possible rearrangement for <code>nums</code> is <code>[2,1,3]</code>, which gives the maximum alternating score among all possible rearrangements.</p>

<p>The alternating score is calculated as:</p>

<p><code>score = 2<sup>2</sup> - 1<sup>2</sup> + 3<sup>2</sup> = 4 - 1 + 9 = 12</code></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,-1,2,-2,3,-3]</span></p>

<p><strong>Output:</strong> <span class="example-io">16</span></p>

<p><strong>Explanation:</strong></p>

<p>A possible rearrangement for <code>nums</code> is <code>[-3,-1,-2,1,3,2]</code>, which gives the maximum alternating score among all possible rearrangements.</p>

<p>The alternating score is calculated as:</p>

<p><code>score = (-3)<sup>2</sup> - (-1)<sup>2</sup> + (-2)<sup>2</sup> - (1)<sup>2</sup> + (3)<sup>2</sup> - (2)<sup>2</sup> = 9 - 1 + 4 - 1 + 9 - 4 = 16</code></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-4 * 10<sup>4</sup> &lt;= nums[i] &lt;= 4 * 10<sup>4</sup></code></li>
</ul>


## Hints

1. The score uses squares of values. The original signs of <code>nums</code> don't affect the squared terms.
2. In the alternating sum, even indices contribute positively and odd indices negatively.

## Solution

```rust
impl Solution { pub fn max_alternating_sum(black_nums: Vec<i32>) -> i64 { let (black_n, mut black_min, mut black_max) = (black_nums.len(), 40000, 0); for &black_x in &black_nums { let black_abs = black_x.abs(); black_min = black_min.min(black_abs); black_max = black_max.max(black_abs); } let mut black_cnt = vec![0; (black_max - black_min + 1) as usize]; for &black_x in &black_nums { black_cnt[(black_x.abs() - black_min) as usize] += 1; } let (mut black_res, mut black_proc, black_half) = (0i64, 0, (black_n + 1) / 2); for black_i in (black_min..=black_max).rev() { let black_sq = black_i as i64 * black_i as i64; for _ in 0..black_cnt[(black_i - black_min) as usize] { if black_proc < black_half { black_res += black_sq; } else { black_res -= black_sq; } black_proc += 1; } } black_res } }
```