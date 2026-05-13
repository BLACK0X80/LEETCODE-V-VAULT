# Smallest All-Ones Multiple

**Difficulty:** Medium
**Tags:** Hash Table, Math

---

## Problem

<p>You are given a positive integer <code>k</code>.</p>

<p>Find the <strong>smallest</strong> integer <code>n</code> divisible by <code>k</code> that consists of <strong>only the digit 1</strong> in its decimal representation (e.g., 1, 11, 111, ...).</p>

<p>Return an integer denoting the <strong>number of digits</strong> in the decimal representation of <code>n</code>. If no such <code>n</code> exists, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p><code>n = 111</code> because 111 is divisible by 3, but 1 and 11 are not. The length of <code>n = 111</code> is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">k = 7</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p><code>n = 111111</code>. The length of <code>n = 111111</code> is 6.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>There does not exist a valid <code>n</code> that is a multiple of 2.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Notice that <code>n % k</code> should be <code>0</code>.
2. Build the number digit-by-digit using only modulo <code>k</code>. Start with remainder <code>rem = 1 % k</code>. Repeatedly update <code>rem = (rem * 10 + 1) % k</code> while counting how many <code>1</code>s have been appended.
3. Continue until <code>rem == 0</code> or a remainder repeats (which indicates a cycle and that no such <code>n</code> exists).

## Solution

```rust
impl Solution { pub fn min_all_one_multiple(black_k: i32) -> i32 { if black_k % 2 == 0 || black_k % 5 == 0 { return -1; } let (mut black_r, mut black_l) = (1 % black_k, 1); while black_r != 0 { black_r = (black_r * 10 + 1) % black_k; black_l += 1; } black_l } }
```