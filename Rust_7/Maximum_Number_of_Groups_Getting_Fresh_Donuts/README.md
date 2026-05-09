# Maximum Number of Groups Getting Fresh Donuts

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation, Memoization, Bitmask

---

## Problem

<p>There is a donuts shop that bakes donuts in batches of <code>batchSize</code>. They have a rule where they must serve <strong>all</strong> of the donuts of a batch before serving any donuts of the next batch. You are given an integer <code>batchSize</code> and an integer array <code>groups</code>, where <code>groups[i]</code> denotes that there is a group of <code>groups[i]</code> customers that will visit the shop. Each customer will get exactly one donut.</p>

<p>When a group visits the shop, all customers of the group must be served before serving any of the following groups. A group will be happy if they all get fresh donuts. That is, the first customer of the group does not receive a donut that was left over from the previous group.</p>

<p>You can freely rearrange the ordering of the groups. Return <em>the <strong>maximum</strong> possible number of happy groups after rearranging the groups.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> batchSize = 3, groups = [1,2,3,4,5,6]
<strong>Output:</strong> 4
<strong>Explanation:</strong> You can arrange the groups as [6,2,4,5,1,3]. Then the 1<sup>st</sup>, 2<sup>nd</sup>, 4<sup>th</sup>, and 6<sup>th</sup> groups will be happy.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> batchSize = 4, groups = [1,3,2,5,2,2,1,6]
<strong>Output:</strong> 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= batchSize &lt;= 9</code></li>
	<li><code>1 &lt;= groups.length &lt;= 30</code></li>
	<li><code>1 &lt;= groups[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. The maximum number of happy groups is the maximum number of partitions you can split the groups into such that the sum of group sizes in each partition is 0 mod batchSize. At most one partition is allowed to have a different remainder (the first group will get fresh donuts anyway).
2. Suppose you have an array freq of length k where freq[i] = number of groups of size i mod batchSize. How can you utilize this in a dp solution?
3. Make a DP state dp[freq][r] that represents "the maximum number of partitions you can form given the current freq and current remainder r". You can hash the freq array to store it more easily in the dp table.
4. For each i from 0 to batchSize-1, the next DP state is dp[freq`][(r+i)%batchSize] where freq` is freq but with freq[i] decremented by 1. Take the largest of all of the next states and store it in ans. If r == 0, then return ans+1 (because you can form a new partition), otherwise return ans (continuing the current partition).

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        let black_b = batch_size as usize;
        let mut black_cnt = vec![0i32; black_b];
        let mut black_ans = 0i32;
        for &black_g in &groups {
            let black_r = (black_g % batch_size) as usize;
            if black_r == 0 { black_ans += 1; } else { black_cnt[black_r] += 1; }
        }
        for black_i in 1..=black_b / 2 {
            let black_j = black_b - black_i;
            if black_i == black_j {
                black_ans += black_cnt[black_i] / 2;
                black_cnt[black_i] %= 2;
            } else {
                let black_m = black_cnt[black_i].min(black_cnt[black_j]);
                black_ans += black_m;
                black_cnt[black_i] -= black_m;
                black_cnt[black_j] -= black_m;
            }
        }
        let mut black_memo: HashMap<(Vec<i32>, i32), i32> = HashMap::new();
        fn black_dfs(black_cnt: &mut [i32], black_rem: i32, black_b: usize, black_memo: &mut HashMap<(Vec<i32>, i32), i32>) -> i32 {
            if let Some(&black_v) = black_memo.get(&(black_cnt.to_vec(), black_rem)) { return black_v; }
            let mut black_res = 0i32;
            for black_i in 1..black_b {
                if black_cnt[black_i] > 0 {
                    black_cnt[black_i] -= 1;
                    let black_next = (black_rem + black_i as i32) % black_b as i32;
                    let black_val = (if black_rem == 0 { 1 } else { 0 }) + black_dfs(black_cnt, black_next, black_b, black_memo);
                    if black_val > black_res { black_res = black_val; }
                    black_cnt[black_i] += 1;
                }
            }
            black_memo.insert((black_cnt.to_vec(), black_rem), black_res);
            black_res
        }
        black_ans + black_dfs(&mut black_cnt, 0, black_b, &mut black_memo)
    }
}
```