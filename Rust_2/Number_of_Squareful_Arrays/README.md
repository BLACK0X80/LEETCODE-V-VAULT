# Number of Squareful Arrays

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Dynamic Programming, Backtracking, Bit Manipulation, Bitmask

---

## Problem

<p>An array is <strong>squareful</strong> if the sum of every pair of adjacent elements is a <strong>perfect square</strong>.</p>

<p>Given an integer array nums, return <em>the number of permutations of </em><code>nums</code><em> that are <strong>squareful</strong></em>.</p>

<p>Two permutations <code>perm1</code> and <code>perm2</code> are different if there is some index <code>i</code> such that <code>perm1[i] != perm2[i]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,17,8]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [1,8,17] and [17,8,1] are the valid permutations.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2,2]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 12</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        
        let is_sq = |a: i64, b: i64| { let s = ((a+b) as f64).sqrt() as i64; s*s == a+b };
        
        let mut dp = vec![vec![0i32; n]; 1<<n];
        for i in 0..n { dp[1<<i][i] = 1; }
        
        for mask in 1..1<<n {
            for last in 0..n {
                if dp[mask][last] == 0 { continue; }
                if mask & (1<<last) == 0 { continue; }
                for next in 0..n {
                    if mask & (1<<next) != 0 { continue; }
                    if !is_sq(nums[last] as i64, nums[next] as i64) { continue; }
                    dp[mask|(1<<next)][next] += dp[mask][last];
                }
            }
        }
        
        let full = (1<<n) - 1;
        let total: i32 = (0..n).map(|i| dp[full][i]).sum();
        
        
        let mut freq = std::collections::HashMap::new();
        for &x in &nums { *freq.entry(x).or_insert(0u64) += 1; }
        let div: u64 = freq.values().map(|&f| (1..=f).product::<u64>()).product();
        
        (total as u64 / div) as i32
    }
}
```