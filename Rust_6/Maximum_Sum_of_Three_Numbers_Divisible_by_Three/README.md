# Maximum Sum of Three Numbers Divisible by Three

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>Your task is to choose <strong>exactly three</strong> integers from <code>nums</code> such that their sum is divisible by three.</p>

<p>Return the <strong>maximum</strong> possible sum of such a triplet. If no such triplet exists, return 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,2,3,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<p>The valid triplets whose sum is divisible by 3 are:</p>

<ul>
	<li><code>(4, 2, 3)</code> with a sum of <code>4 + 2 + 3 = 9</code>.</li>
	<li><code>(2, 3, 1)</code> with a sum of <code>2 + 3 + 1 = 6</code>.</li>
</ul>

<p>Thus, the answer is 9.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>No triplet forms a sum divisible by 3, so the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Split numbers into groups by <code>x % 3</code>.
2. Only four valid combinations for <code>sum % 3 == 0</code>.
3. Possible combinations are <code>0 + 0 + 0</code>, <code>1 + 1 + 1</code>, <code>2 + 2 + 2</code>, <code>0 + 1 + 2</code>.
4. Sort groups descending, try each combo using top values.

## Solution

```rust
impl Solution { pub fn maximum_sum(nums: Vec<i32>) -> i32 { let mut black_r = vec![Vec::new(); 3]; for black_x in nums { black_r[(black_x % 3) as usize].push(black_x); } for black_v in &mut black_r { black_v.sort_unstable_by(|black_a, black_b| black_b.cmp(black_a)); } let mut black_ans = 0; if black_r[0].len() >= 3 { black_ans = black_ans.max(black_r[0][0] + black_r[0][1] + black_r[0][2]); } if black_r[1].len() >= 3 { black_ans = black_ans.max(black_r[1][0] + black_r[1][1] + black_r[1][2]); } if black_r[2].len() >= 3 { black_ans = black_ans.max(black_r[2][0] + black_r[2][1] + black_r[2][2]); } if black_r[0].len() >= 1 && black_r[1].len() >= 1 && black_r[2].len() >= 1 { black_ans = black_ans.max(black_r[0][0] + black_r[1][0] + black_r[2][0]); } black_ans } }
```