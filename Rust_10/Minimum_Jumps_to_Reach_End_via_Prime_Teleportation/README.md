# Minimum Jumps to Reach End via Prime Teleportation

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Breadth-First Search, Number Theory

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code>.</p>

<p>You start at index 0, and your goal is to reach index <code>n - 1</code>.</p>

<p>From any index <code>i</code>, you may perform one of the following operations:</p>

<ul>
	<li><strong>Adjacent Step</strong>: Jump to index <code>i + 1</code> or <code>i - 1</code>, if the index is within bounds.</li>
	<li><strong>Prime Teleportation</strong>: If <code>nums[i]</code> is a <span data-keyword="prime-number">prime number</span> <code>p</code>, you may instantly jump to any index <code>j != i</code> such that <code>nums[j] % p == 0</code>.</li>
</ul>

<p>Return the <strong>minimum</strong> number of jumps required to reach index <code>n - 1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,4,6]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal sequence of jumps is:</p>

<ul>
	<li>Start at index <code>i = 0</code>. Take an adjacent step to index 1.</li>
	<li>At index <code>i = 1</code>, <code>nums[1] = 2</code> is a prime number. Therefore, we teleport to index <code>i = 3</code> as <code>nums[3] = 6</code> is divisible by 2.</li>
</ul>

<p>Thus, the answer is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,4,7,9]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal sequence of jumps is:</p>

<ul>
	<li>Start at index <code>i = 0</code>. Take an adjacent step to index <code>i = 1</code>.</li>
	<li>At index <code>i = 1</code>, <code>nums[1] = 3</code> is a prime number. Therefore, we teleport to index <code>i = 4</code> since <code>nums[4] = 9</code> is divisible by 3.</li>
</ul>

<p>Thus, the answer is 2.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,6,5,8]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Since no teleportation is possible, we move through <code>0 &rarr; 1 &rarr; 2 &rarr; 3</code>. Thus, the answer is 3.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Use a breadth-first search.
2. Precompute prime factors of each <code>nums[i]</code> via a sieve, and build a bucket <code>bucket[p]</code> mapping each prime <code>p</code> to all indices <code>j</code> with <code>nums[j] % p == 0</code>.
3. During the BFS, when at index <code>i</code>, enqueue its adjacent steps (<code>i+1</code> and <code>i-1</code>) and all indices in <code>bucket[p]</code> for each prime <code>p</code> dividing <code>nums[i]</code>, then clear <code>bucket[p]</code> so each prime's bucket is visited only once.

## Solution

```rust
use std::collections::VecDeque; impl Solution { pub fn min_jumps(black_nums: Vec<i32>) -> i32 { let (black_n, black_max) = (black_nums.len(), *black_nums.iter().max().unwrap() as usize); let mut black_is_p = vec![true; black_max + 1]; if black_max >= 1 { black_is_p[0] = false; black_is_p[1] = false; } for black_i in 2..=((black_max as f64).sqrt() as usize) { if black_is_p[black_i] { for black_j in (black_i * black_i..=black_max).step_by(black_i) { black_is_p[black_j] = false; } } } let mut black_p_to_idx = vec![vec![]; black_max + 1]; for (black_i, &black_v) in black_nums.iter().enumerate() { let mut black_v = black_v as usize; for black_p in 2..=((black_v as f64).sqrt() as usize) { if black_v % black_p == 0 { black_p_to_idx[black_p].push(black_i); while black_v % black_p == 0 { black_v /= black_p; } } } if black_v > 1 { black_p_to_idx[black_v].push(black_i); } } let (mut black_q, mut black_v_idx, mut black_v_p) = (VecDeque::from([(0, 0)]), vec![false; black_n], vec![false; black_max + 1]); black_v_idx[0] = true; while let Some((black_curr, black_dist)) = black_q.pop_front() { if black_curr == black_n - 1 { return black_dist; } for black_next in [black_curr as i32 - 1, black_curr as i32 + 1] { if black_next >= 0 && black_next < black_n as i32 && !black_v_idx[black_next as usize] { black_v_idx[black_next as usize] = true; black_q.push_back((black_next as usize, black_dist + 1)); } } let black_val = black_nums[black_curr] as usize; if black_is_p[black_val] { if !black_v_p[black_val] { black_v_p[black_val] = true; for &black_target in &black_p_to_idx[black_val] { if !black_v_idx[black_target] { black_v_idx[black_target] = true; black_q.push_back((black_target, black_dist + 1)); } } } } } -1 } }
```