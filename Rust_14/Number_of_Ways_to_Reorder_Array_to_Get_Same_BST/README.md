# Number of Ways to Reorder Array to Get Same BST

**Difficulty:** Hard
**Tags:** Array, Math, Divide and Conquer, Dynamic Programming, Tree, Union-Find, Binary Search Tree, Memoization, Combinatorics, Binary Tree

---

## Problem

<p>Given an array <code>nums</code> that represents a permutation of integers from <code>1</code> to <code>n</code>. We are going to construct a binary search tree (BST) by inserting the elements of <code>nums</code> in order into an initially empty BST. Find the number of different ways to reorder <code>nums</code> so that the constructed BST is identical to that formed from the original array <code>nums</code>.</p>

<ul>
	<li>For example, given <code>nums = [2,1,3]</code>, we will have 2 as the root, 1 as a left child, and 3 as a right child. The array <code>[2,3,1]</code> also yields the same BST but <code>[3,2,1]</code> yields a different BST.</li>
</ul>

<p>Return <em>the number of ways to reorder</em> <code>nums</code> <em>such that the BST formed is identical to the original BST formed from</em> <code>nums</code>.</p>

<p>Since the answer may be very large, <strong>return it modulo </strong><code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2026/02/23/1569_ex0.png" style="width: 160px; height: 160px;" />
<pre>
<strong>Input:</strong> nums = [2,1,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can reorder nums to be [2,3,1] which will yield the same BST. There are no other ways to reorder nums which will yield the same BST.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2026/02/23/1569_ex.png" style="width: 270px; height: 270px;" />
<pre>
<strong>Input:</strong> nums = [3,4,5,1,2]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The following 5 arrays will yield the same BST: 
[3,1,2,4,5]
[3,1,4,2,5]
[3,1,4,5,2]
[3,4,1,2,5]
[3,4,1,5,2]
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2026/02/23/1569_ex2.png" style="width: 205px; height: 205px;" />
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no other orderings of nums that will yield the same BST.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= nums.length</code></li>
	<li>All integers in <code>nums</code> are <strong>distinct</strong>.</li>
</ul>


## Hints

1. Use a divide and conquer strategy.
2. The first number will always be the root. Consider the numbers smaller and larger than the root separately. When merging the results together, how many ways can you order x elements in x+y positions?

## Solution

```rust
impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let n = nums.len();
        let mut black_c = vec![vec![0u64; n + 1]; n + 1];
        for i in 0..=n { black_c[i][0] = 1; for j in 1..=i { black_c[i][j] = (black_c[i-1][j-1] + black_c[i-1][j]) % MOD; } }

        fn black_dfs(black_nums: Vec<i32>, black_c: &Vec<Vec<u64>>) -> u64 {
            if black_nums.len() <= 1 { return 1; }
            const MOD: u64 = 1_000_000_007;
            let root = black_nums[0];
            let black_left: Vec<i32> = black_nums[1..].iter().filter(|&&x| x < root).cloned().collect();
            let black_right: Vec<i32> = black_nums[1..].iter().filter(|&&x| x > root).cloned().collect();
            let (l, r) = (black_left.len(), black_right.len());
            black_c[l+r][l] % MOD * black_dfs(black_left, black_c) % MOD * black_dfs(black_right, black_c) % MOD
        }

        ((black_dfs(nums, &black_c) - 1 + MOD) % MOD) as i32
    }
}
```