# Minimum Difficulty of a Job Schedule

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You want to schedule a list of jobs in <code>d</code> days. Jobs are dependent (i.e To work on the <code>i<sup>th</sup></code> job, you have to finish all the jobs <code>j</code> where <code>0 &lt;= j &lt; i</code>).</p>

<p>You have to finish <strong>at least</strong> one task every day. The difficulty of a job schedule is the sum of difficulties of each day of the <code>d</code> days. The difficulty of a day is the maximum difficulty of a job done on that day.</p>

<p>You are given an integer array <code>jobDifficulty</code> and an integer <code>d</code>. The difficulty of the <code>i<sup>th</sup></code> job is <code>jobDifficulty[i]</code>.</p>

<p>Return <em>the minimum difficulty of a job schedule</em>. If you cannot find a schedule for the jobs return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/untitled.png" style="width: 365px; height: 370px;" />
<pre>
<strong>Input:</strong> jobDifficulty = [6,5,4,3,2,1], d = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> First day you can finish the first 5 jobs, total difficulty = 6.
Second day you can finish the last job, total difficulty = 1.
The difficulty of the schedule = 6 + 1 = 7 
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> jobDifficulty = [9,9,9], d = 4
<strong>Output:</strong> -1
<strong>Explanation:</strong> If you finish a job per day you will still have a free day. you cannot find a schedule for the given jobs.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> jobDifficulty = [1,1,1], d = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The schedule is one job per day. total difficulty will be 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= jobDifficulty.length &lt;= 300</code></li>
	<li><code>0 &lt;= jobDifficulty[i] &lt;= 1000</code></li>
	<li><code>1 &lt;= d &lt;= 10</code></li>
</ul>


## Hints

1. Use DP. Try to cut the array into d non-empty sub-arrays. Try all possible cuts for the array.
2. Use dp[i][j] where DP states are i the index of the last cut and j the number of remaining cuts. Complexity is O(n * n * d).

## Solution

```rust
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (black_n, black_d) = (job_difficulty.len(), d as usize);
        if black_n < black_d { return -1; }
        let mut black_dp = vec![vec![10000; black_n + 1]; black_d + 1];
        black_dp[0][0] = 0;
        for black_i in 1..=black_d {
            for black_j in black_i..=black_n {
                let mut black_mx = 0;
                for black_k in (black_i - 1..black_j).rev() {
                    black_mx = black_mx.max(job_difficulty[black_k]);
                    black_dp[black_i][black_j] = black_dp[black_i][black_j].min(black_dp[black_i - 1][black_k] + black_mx);
                }
            }
        }
        black_dp[black_d][black_n]
    }
}
```