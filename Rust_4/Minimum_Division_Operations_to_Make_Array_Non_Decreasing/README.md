# Minimum Division Operations to Make Array Non Decreasing

**Difficulty:** Medium
**Tags:** Array, Math, Greedy, Number Theory

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>Any <strong>positive</strong> divisor of a natural number <code>x</code> that is <strong>strictly less</strong> than <code>x</code> is called a <strong>proper divisor</strong> of <code>x</code>. For example, 2 is a <em>proper divisor</em> of 4, while 6 is not a <em>proper divisor</em> of 6.</p>

<p>You are allowed to perform an <strong>operation</strong> any number of times on <code>nums</code>, where in each <strong>operation</strong> you select any <em>one</em> element from <code>nums</code> and divide it by its <strong>greatest</strong> <strong>proper divisor</strong>.</p>

<p>Return the <strong>minimum</strong> number of <strong>operations</strong> required to make the array <strong>non-decreasing</strong>.</p>

<p>If it is <strong>not</strong> possible to make the array <em>non-decreasing</em> using any number of operations, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [25,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>Using a single operation, 25 gets divided by 5 and <code>nums</code> becomes <code>[5, 7]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [7,7,6]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Iterate backward from the last index.
2. Each number can be divided by its largest proper divisor to yield its smallest prime divisor.

## Solution

```rust
impl Solution { pub fn min_operations(mut nums: Vec<i32>) -> i32 { let (mut black_ans, mut black_lpf) = (0, vec![0; 1000001]); for i in 2..1001 { if black_lpf[i] == 0 { for j in (i*i..1000001).step_by(i) { if black_lpf[j] == 0 { black_lpf[j] = i as i32; } } } } for i in (0..nums.len()-1).rev() { while nums[i] > nums[i+1] { let black_d = black_lpf[nums[i] as usize]; if black_d == 0 || black_d == nums[i] { return -1; } nums[i] = black_d; black_ans += 1; } } black_ans } }
```