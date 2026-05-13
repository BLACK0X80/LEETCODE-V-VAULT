# Beautiful Towers I

**Difficulty:** Medium
**Tags:** Array, Stack, Monotonic Stack

---

## Problem

<p>You are given an array <code>heights</code> of <code>n</code> integers representing the number of bricks in <code>n</code> consecutive towers. Your task is to remove some bricks to form a <strong>mountain-shaped</strong> tower arrangement. In this arrangement, the tower heights are non-decreasing, reaching a maximum peak value with one or multiple consecutive towers and then non-increasing.</p>

<p>Return the <strong>maximum possible sum</strong> of heights of a mountain-shaped tower arrangement.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">heights = [5,3,4,1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">13</span></p>

<p><strong>Explanation:</strong></p>

<p>We remove some bricks to make <code>heights =&nbsp;[5,3,3,1,1]</code>, the peak is at index 0.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">heights = [6,5,3,9,2,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">22</span></p>

<p><strong>Explanation:</strong></p>

<p>We remove some bricks to make <code>heights =&nbsp;[3,3,3,9,2,2]</code>, the peak is at index 3.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">heights = [3,2,5,5,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">18</span></p>

<p><strong>Explanation:</strong></p>

<p>We remove some bricks to make <code>heights = [2,2,5,5,2,2]</code>, the peak is at index 2 or 3.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == heights.length &lt;= 10<sup>3</sup></code></li>
	<li><code>1 &lt;= heights[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Try all the possible indices <code>i</code> as the peak.
2. If <code>i</code> is the peak, <code>i-1<sup>th</sup></code> element, and <code>heights[j] = min(heights[j], heights[j + 1])</code> for <code>0 <= j < i </code>
3. If <code>i</code> is the peak, start from <code>i+1<sup>th</sup></code> element, heights[j] = min(heights[j], heights[j - 1]) for <code>i < j < heights.size()</code>

## Solution

```rust
impl Solution { pub fn maximum_sum_of_heights(black_h: Vec<i32>) -> i64 { let black_n = black_h.len(); let black_f = |black_v: &Vec<i32>| { let (mut black_s, mut black_res, mut black_cur) = (vec![(-1isize, 0i64)], vec![0i64; black_n], 0i64); for black_i in 0..black_n { while let Some(&(black_p, _)) = black_s.last() { if black_p != -1 && black_v[black_p as usize] >= black_v[black_i] { black_cur -= black_s.pop().unwrap().1; } else { break; } } let black_prev = black_s.last().unwrap().0; let black_val = (black_i as isize - black_prev) as i64 * black_v[black_i] as i64; black_cur += black_val; black_res[black_i] = black_cur; black_s.push((black_i as isize, black_val)); } black_res }; let (black_l, mut black_rv) = (black_f(&black_h), black_h.clone()); black_rv.reverse(); let mut black_r = black_f(&black_rv); black_r.reverse(); (0..black_n).map(|black_i| black_l[black_i] + black_r[black_i] - black_h[black_i] as i64).max().unwrap() } }
```