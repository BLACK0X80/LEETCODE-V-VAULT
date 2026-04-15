# Find the Minimum Cost Array Permutation

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation, Bitmask

---

## Problem

<p>You are given an array <code>nums</code> which is a <span data-keyword="permutation">permutation</span> of <code>[0, 1, 2, ..., n - 1]</code>. The <strong>score</strong> of any permutation of <code>[0, 1, 2, ..., n - 1]</code> named <code>perm</code> is defined as:</p>

<p><code>score(perm) = |perm[0] - nums[perm[1]]| + |perm[1] - nums[perm[2]]| + ... + |perm[n - 1] - nums[perm[0]]|</code></p>

<p>Return the permutation <code>perm</code> which has the <strong>minimum</strong> possible score. If <em>multiple</em> permutations exist with this score, return the one that is <span data-keyword="lexicographically-smaller-array">lexicographically smallest</span> among them.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,0,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,1,2]</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/04/04/example0gif.gif" style="width: 235px; height: 235px;" /></strong></p>

<p>The lexicographically smallest permutation with minimum cost is <code>[0,1,2]</code>. The cost of this permutation is <code>|0 - 0| + |1 - 2| + |2 - 1| = 2</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,2,1]</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/04/04/example1gif.gif" style="width: 235px; height: 235px;" /></strong></p>

<p>The lexicographically smallest permutation with minimum cost is <code>[0,2,1]</code>. The cost of this permutation is <code>|0 - 1| + |2 - 2| + |1 - 0| = 2</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums.length &lt;= 14</code></li>
	<li><code>nums</code> is a permutation of <code>[0, 1, 2, ..., n - 1]</code>.</li>
</ul>


## Hints

1. The score function is cyclic, so we can always set <code>perm[0] = 0</code> for the smallest lexical order.
2. It’s similar to the Traveling Salesman Problem. Use Dynamic Programming.
3. Use a bitmask to track which elements have been assigned to <code>perm</code>.

## Solution

```rust
impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let black_n = nums.len();
        let mut black_dp = vec![vec![i32::MAX; black_n]; 1 << black_n];
        let mut black_parent = vec![vec![0usize; black_n]; 1 << black_n];

        for i in 0..black_n {
            black_dp[(1 << black_n) - 1][i] = (i as i32 - nums[0]).abs();
        }

        for mask in (1..(1 << black_n) - 1).rev() {
            for last in 0..black_n {
                if (mask & (1 << last)) == 0 { continue; }
                for next in 0..black_n {
                    if (mask & (1 << next)) != 0 { continue; }
                    let cost = (last as i32 - nums[next]).abs() + black_dp[mask | (1 << next)][next];
                    if cost < black_dp[mask][last] {
                        black_dp[mask][last] = cost;
                        black_parent[mask][last] = next;
                    }
                }
            }
        }

        let mut black_res = vec![0];
        let mut black_m = 1usize;
        let mut black_l = 0usize;
        while black_res.len() < black_n {
            let next = black_parent[black_m][black_l];
            black_res.push(next as i32);
            black_m |= 1 << next;
            black_l = next;
        }
        black_res
    }
}
```