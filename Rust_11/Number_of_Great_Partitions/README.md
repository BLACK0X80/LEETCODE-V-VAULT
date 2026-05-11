# Number of Great Partitions

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an array <code>nums</code> consisting of <strong>positive</strong> integers and an integer <code>k</code>.</p>

<p><strong>Partition</strong> the array into two ordered <strong>groups</strong> such that each element is in exactly <strong>one</strong> group. A partition is called great if the <strong>sum</strong> of elements of each group is greater than or equal to <code>k</code>.</p>

<p>Return <em>the number of <strong>distinct</strong> great partitions</em>. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>Two partitions are considered distinct if some element <code>nums[i]</code> is in different groups in the two partitions.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The great partitions are: ([1,2,3], [4]), ([1,3], [2,4]), ([1,4], [2,3]), ([2,3], [1,4]), ([2,4], [1,3]) and ([4], [1,2,3]).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,3,3], k = 4
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no great partitions for this array.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [6,6], k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can either put nums[0] in the first partition or in the second partition.
The great partitions will be ([6], [6]) and ([6], [6]).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length, k &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. If the sum of the array is smaller than 2*k, then it is impossible to find a great partition.
2. Solve the reverse problem, that is, find the number of partitions where the sum of elements of at least one of the two groups is smaller than k.

## Solution

```rust
impl Solution {
    pub fn count_partitions(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let black_sum: i64 = black_nums.iter().map(|&x| x as i64).sum();
        if black_sum < 2 * black_k as i64 { return 0; }
        let black_mod = 1_000_000_007i64;
        let mut black_dp = vec![0i64; black_k as usize];
        black_dp[0] = 1;

        for &black_num in &black_nums {
            for black_i in (black_num as usize..black_k as usize).rev() {
                black_dp[black_i] = (black_dp[black_i] + black_dp[black_i - black_num as usize]) % black_mod;
            }
        }

        let mut black_total_pow = 1i64;
        for _ in 0..black_nums.len() { black_total_pow = (black_total_pow * 2) % black_mod; }
        let bravexuneth = black_dp.iter().sum::<i64>() % black_mod;
        ((black_total_pow - 2 * bravexuneth + 2 * black_mod) % black_mod) as i32
    }
}
```