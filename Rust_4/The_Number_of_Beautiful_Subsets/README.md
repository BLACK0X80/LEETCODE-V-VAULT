# The Number of Beautiful Subsets

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Dynamic Programming, Backtracking, Sorting, Combinatorics

---

## Problem

<p>You are given an array <code>nums</code> of positive integers and a <strong>positive</strong> integer <code>k</code>.</p>

<p>A subset of <code>nums</code> is <strong>beautiful</strong> if it does not contain two integers with an absolute difference equal to <code>k</code>.</p>

<p>Return <em>the number of <strong>non-empty beautiful </strong>subsets of the array</em> <code>nums</code>.</p>

<p>A <strong>subset</strong> of <code>nums</code> is an array that can be obtained by deleting some (possibly none) elements from <code>nums</code>. Two subsets are different if and only if the chosen indices to delete are different.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,4,6], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The beautiful subsets of the array nums are: [2], [4], [6], [2, 6].
It can be proved that there are only 4 beautiful subsets in the array [2,4,6].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1], k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The beautiful subset of the array nums is [1].
It can be proved that there is only 1 beautiful subset in the array [1].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 18</code></li>
	<li><code>1 &lt;= nums[i], k &lt;= 1000</code></li>
</ul>


## Hints

1. Sort the array nums and create another array cnt of size nums[i].
2. Use backtracking to generate all the beautiful subsets. If cnt[nums[i] - k] is positive, then it is impossible to add nums[i] in the subset, and we just move to the next index. Otherwise, it is also possible to add nums[i] in the subset, in this case, increase cnt[nums[i]], and move to the next index.
3. Bonus: Can you solve the problem in O(n log n)?

## Solution

```rust
impl Solution { pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 { fn black_dfs(i: usize, n: &Vec<i32>, k: i32, m: &mut [i32; 2001]) -> i32 { if i == n.len() { return 1; } let mut r = black_dfs(i + 1, n, k, m); if (n[i]-k < 0 || m[(n[i]-k) as usize] == 0) && (n[i]+k > 2000 || m[(n[i]+k) as usize] == 0) { m[n[i] as usize] += 1; r += black_dfs(i + 1, n, k, m); m[n[i] as usize] -= 1; } r } black_dfs(0, &nums, k, &mut [0; 2001]) - 1 } }
```