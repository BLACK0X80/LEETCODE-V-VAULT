# Earliest Possible Day of Full Bloom

**Difficulty:** Hard
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You have <code>n</code> flower seeds. Every seed must be planted first before it can begin to grow, then bloom. Planting a seed takes time and so does the growth of a seed. You are given two <strong>0-indexed</strong> integer arrays <code>plantTime</code> and <code>growTime</code>, of length <code>n</code> each:</p>

<ul>
	<li><code>plantTime[i]</code> is the number of <strong>full days</strong> it takes you to <strong>plant</strong> the <code>i<sup>th</sup></code> seed. Every day, you can work on planting exactly one seed. You <strong>do not</strong> have to work on planting the same seed on consecutive days, but the planting of a seed is not complete <strong>until</strong> you have worked <code>plantTime[i]</code> days on planting it in total.</li>
	<li><code>growTime[i]</code> is the number of <strong>full days</strong> it takes the <code>i<sup>th</sup></code> seed to grow after being completely planted. <strong>After</strong> the last day of its growth, the flower <strong>blooms</strong> and stays bloomed forever.</li>
</ul>

<p>From the beginning of day <code>0</code>, you can plant the seeds in <strong>any</strong> order.</p>

<p>Return <em>the <strong>earliest</strong> possible day where <strong>all</strong> seeds are blooming</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/1.png" style="width: 453px; height: 149px;" />
<pre>
<strong>Input:</strong> plantTime = [1,4,3], growTime = [2,3,1]
<strong>Output:</strong> 9
<strong>Explanation:</strong> The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
One optimal way is:
On day 0, plant the 0<sup>th</sup> seed. The seed grows for 2 full days and blooms on day 3.
On days 1, 2, 3, and 4, plant the 1<sup>st</sup> seed. The seed grows for 3 full days and blooms on day 8.
On days 5, 6, and 7, plant the 2<sup>nd</sup> seed. The seed grows for 1 full day and blooms on day 9.
Thus, on day 9, all the seeds are blooming.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/2.png" style="width: 454px; height: 184px;" />
<pre>
<strong>Input:</strong> plantTime = [1,2,3,2], growTime = [2,1,2,1]
<strong>Output:</strong> 9
<strong>Explanation:</strong> The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
One optimal way is:
On day 1, plant the 0<sup>th</sup> seed. The seed grows for 2 full days and blooms on day 4.
On days 0 and 3, plant the 1<sup>st</sup> seed. The seed grows for 1 full day and blooms on day 5.
On days 2, 4, and 5, plant the 2<sup>nd</sup> seed. The seed grows for 2 full days and blooms on day 8.
On days 6 and 7, plant the 3<sup>rd</sup> seed. The seed grows for 1 full day and blooms on day 9.
Thus, on day 9, all the seeds are blooming.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> plantTime = [1], growTime = [1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> On day 0, plant the 0<sup>th</sup> seed. The seed grows for 1 full day and blooms on day 2.
Thus, on day 2, all the seeds are blooming.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == plantTime.length == growTime.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= plantTime[i], growTime[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. List the planting like the diagram above shows, where a row represents the timeline of a seed. A row i is above another row j if the last day planting seed i is ahead of the last day for seed j. Does it have any advantage to spend some days to plant seed j before completely planting seed i?
2. No. It does not help seed j but could potentially delay the completion of seed i, resulting in a worse final answer. Remaining focused is a part of the optimal solution.
3. Sort the seeds by their growTime in descending order. Can you prove why this strategy is the other part of the optimal solution? Note the bloom time of a seed is the sum of plantTime of all seeds preceding this seed plus the growTime of this seed.
4. There is no way to improve this strategy. The seed to bloom last dominates the final answer. Exchanging the planting of this seed with another seed with either a larger or smaller growTime will result in a potentially worse answer.

## Solution

```rust
impl Solution {
    pub fn earliest_full_bloom(black1: Vec<i32>, black2: Vec<i32>) -> i32 {
        let mut black3: Vec<usize> = (0..black1.len()).collect();
        black3.sort_unstable_by_key(|&i| -black2[i]);
        let (mut black4, mut black5) = (0, 0);
        for black6 in black3 {
            black5 += black1[black6];
            black4 = black4.max(black5 + black2[black6]);
        }
        black4
    }
}
```