# Make Lexicographically Smallest Array by Swapping Elements

**Difficulty:** Medium
**Tags:** Array, Union-Find, Sorting

---

## Problem

<p>You are given a <strong>0-indexed</strong> array of <strong>positive</strong> integers <code>nums</code> and a <strong>positive</strong> integer <code>limit</code>.</p>

<p>In one operation, you can choose any two indices <code>i</code> and <code>j</code> and swap <code>nums[i]</code> and <code>nums[j]</code> <strong>if</strong> <code>|nums[i] - nums[j]| &lt;= limit</code>.</p>

<p>Return <em>the <strong>lexicographically smallest array</strong> that can be obtained by performing the operation any number of times</em>.</p>

<p>An array <code>a</code> is lexicographically smaller than an array <code>b</code> if in the first position where <code>a</code> and <code>b</code> differ, array <code>a</code> has an element that is less than the corresponding element in <code>b</code>. For example, the array <code>[2,10,3]</code> is lexicographically smaller than the array <code>[10,2,3]</code> because they differ at index <code>0</code> and <code>2 &lt; 10</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,5,3,9,8], limit = 2
<strong>Output:</strong> [1,3,5,8,9]
<strong>Explanation:</strong> Apply the operation 2 times:
- Swap nums[1] with nums[2]. The array becomes [1,3,5,9,8]
- Swap nums[3] with nums[4]. The array becomes [1,3,5,8,9]
We cannot obtain a lexicographically smaller array by applying any more operations.
Note that it may be possible to get the same result by doing different operations.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,7,6,18,2,1], limit = 3
<strong>Output:</strong> [1,6,7,18,1,2]
<strong>Explanation:</strong> Apply the operation 3 times:
- Swap nums[1] with nums[2]. The array becomes [1,6,7,18,2,1]
- Swap nums[0] with nums[4]. The array becomes [2,6,7,18,1,1]
- Swap nums[0] with nums[5]. The array becomes [1,6,7,18,1,2]
We cannot obtain a lexicographically smaller array by applying any more operations.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,7,28,19,10], limit = 3
<strong>Output:</strong> [1,7,28,19,10]
<strong>Explanation:</strong> [1,7,28,19,10] is the lexicographically smallest array we can obtain because we cannot apply the operation on any two indices.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= limit &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Construct a virtual graph where all elements in <code>nums</code> are nodes and the pairs satisfying the condition have an edge between them.
2. Instead of constructing all edges, we only care about the connected components.
3. Can we use DSU?
4. Sort <code>nums</code>. Now we just need to consider if the consecutive elements have an edge to check if they belong to the same connected component. Hence, all connected components become a list of position-consecutive elements after sorting.
5. For each index of <code>nums</code> from <code>0</code> to <code>nums.length - 1</code> we can change it to the current minimum value we have in its connected component and remove that value from the connected component.

## Solution

```rust
impl Solution { pub fn lexicographically_smallest_array(black_n: Vec<i32>, black_l: i32) -> Vec<i32> { let mut black_v: Vec<_> = black_n.iter().enumerate().map(|(i, &x)| (x, i)).collect(); black_v.sort_unstable(); let mut black_res = vec![0; black_n.len()]; let mut black_i = 0; while black_i < black_v.len() { let mut black_j = black_i + 1; while black_j < black_v.len() && black_v[black_j].0 - black_v[black_j-1].0 <= black_l { black_j += 1; } let mut black_idx: Vec<_> = black_v[black_i..black_j].iter().map(|&(_, idx)| idx).collect(); black_idx.sort_unstable(); for (k, &idx) in black_idx.iter().enumerate() { black_res[idx] = black_v[black_i + k].0; } black_i = black_j; } black_res } }
```