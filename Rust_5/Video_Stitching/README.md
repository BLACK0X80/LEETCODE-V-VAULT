# Video Stitching

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Greedy

---

## Problem

<p>You are given a series of video clips from a sporting event that lasted <code>time</code> seconds. These video clips can be overlapping with each other and have varying lengths.</p>

<p>Each video clip is described by an array <code>clips</code> where <code>clips[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> indicates that the ith clip started at <code>start<sub>i</sub></code> and ended at <code>end<sub>i</sub></code>.</p>

<p>We can cut these clips into segments freely.</p>

<ul>
	<li>For example, a clip <code>[0, 7]</code> can be cut into segments <code>[0, 1] + [1, 3] + [3, 7]</code>.</li>
</ul>

<p>Return <em>the minimum number of clips needed so that we can cut the clips into segments that cover the entire sporting event</em> <code>[0, time]</code>. If the task is impossible, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]], time = 10
<strong>Output:</strong> 3
<strong>Explanation:</strong> We take the clips [0,2], [8,10], [1,9]; a total of 3 clips.
Then, we can reconstruct the sporting event as follows:
We cut [1,9] into segments [1,2] + [2,8] + [8,9].
Now we have segments [0,2] + [2,8] + [8,10] which cover the sporting event [0, 10].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> clips = [[0,1],[1,2]], time = 5
<strong>Output:</strong> -1
<strong>Explanation:</strong> We cannot cover [0,5] with only [0,1] and [1,2].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> clips = [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]], time = 9
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can take clips [0,4], [4,7], and [6,9].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= clips.length &lt;= 100</code></li>
	<li><code>0 &lt;= start<sub>i</sub> &lt;= end<sub>i</sub> &lt;= 100</code></li>
	<li><code>1 &lt;= time &lt;= 100</code></li>
</ul>


## Hints

1. What if we sort the intervals?  Considering the sorted intervals, how can we solve the problem with dynamic programming?
2. Let's consider a DP(pos, limit) where pos represents the position of the current interval we are gonna take the decision and limit is the current covered area from [0 - limit]. This DP returns the minimum number of taken intervals or infinite if it's not possible to cover the [0 - T] section.

## Solution

```rust
impl Solution { pub fn video_stitching(black_clips: Vec<Vec<i32>>, black_time: i32) -> i32 { let mut black_dp = vec![101; black_time as usize + 1]; black_dp[0] = 0; for black_i in 1..=black_time as usize { for black_c in &black_clips { if (black_c[0] as usize) < black_i && black_i <= (black_c[1] as usize) { black_dp[black_i] = black_dp[black_i].min(black_dp[black_c[0] as usize] + 1); } } } if black_dp[black_time as usize] > 100 { -1 } else { black_dp[black_time as usize] } } }
```