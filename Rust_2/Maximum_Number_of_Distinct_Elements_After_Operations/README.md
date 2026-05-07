# Maximum Number of Distinct Elements After Operations

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>k</code>.</p>

<p>You are allowed to perform the following <strong>operation</strong> on each element of the array <strong>at most</strong> <em>once</em>:</p>

<ul>
	<li>Add an integer in the range <code>[-k, k]</code> to the element.</li>
</ul>

<p>Return the <strong>maximum</strong> possible number of <strong>distinct</strong> elements in <code>nums</code> after performing the <strong>operations</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,2,3,3,4], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p><code>nums</code> changes to <code>[-1, 0, 1, 2, 3, 4]</code> after performing operations on the first four elements.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,4,4,4], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>By adding -1 to <code>nums[0]</code> and 1 to <code>nums[1]</code>, <code>nums</code> changes to <code>[3, 5, 4, 4]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Can we use sorting here?
2. Find the minimum element which is not used for each element.

## Solution

```rust
impl Solution { pub fn max_distinct_elements(mut black_n: Vec<i32>, k: i32) -> i32 { black_n.sort_unstable(); let (mut black_last, mut black_ans) = (i32::MIN, 0); for x in black_n { let black_v = (black_last as i64 + 1).max(x as i64 - k as i64); if black_v <= x as i64 + k as i64 { black_last = black_v as i32; black_ans += 1; } } black_ans } }
```