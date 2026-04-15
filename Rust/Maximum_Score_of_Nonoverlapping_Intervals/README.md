# Maximum Score of Non-overlapping Intervals

**Difficulty:** Hard
**Tags:** Array, Binary Search, Dynamic Programming, Sorting

---

## Problem

<p>You are given a 2D integer array <code>intervals</code>, where <code>intervals[i] = [l<sub>i</sub>, r<sub>i</sub>, weight<sub>i</sub>]</code>. Interval <code>i</code> starts at position <code>l<sub>i</sub></code> and ends at <code>r<sub>i</sub></code>, and has a weight of <code>weight<sub>i</sub></code>. You can choose <em>up to</em> 4 <strong>non-overlapping</strong> intervals. The <strong>score</strong> of the chosen intervals is defined as the total sum of their weights.</p>

<p>Return the <span data-keyword="lexicographically-smaller-array">lexicographically smallest</span> array of at most 4 indices from <code>intervals</code> with <strong>maximum</strong> score, representing your choice of non-overlapping intervals.</p>

<p>Two intervals are said to be <strong>non-overlapping</strong> if they do not share any points. In particular, intervals sharing a left or right boundary are considered overlapping.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">intervals = [[1,3,2],[4,5,2],[1,5,5],[6,9,3],[6,7,1],[8,9,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,3]</span></p>

<p><strong>Explanation:</strong></p>

<p>You can choose the intervals with indices 2, and 3 with respective weights of 5, and 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">intervals = [[5,8,1],[6,7,7],[4,7,3],[9,10,6],[7,8,2],[11,14,3],[3,5,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,3,5,6]</span></p>

<p><strong>Explanation:</strong></p>

<p>You can choose the intervals with indices 1, 3, 5, and 6 with respective weights of 7, 6, 3, and 5.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= intevals.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>intervals[i].length == 3</code></li>
	<li><code>intervals[i] = [l<sub>i</sub>, r<sub>i</sub>, weight<sub>i</sub>]</code></li>
	<li><code>1 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= weight<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use Dynamic Programming.
2. Sort <code>intervals</code> by right boundary.
3. Let <code>dp[r][i]</code> denote the maximum score having picked <code>r</code> intervals from the prefix of <code>intervals</code> ending at index <code>i</code>.
4. <code>dp[r][i] = max(dp[r][i - 1], intervals[i][2] + dp[r][j])</code> where <code>j</code> is the largest index such that <code>intervals[j][1] < intervals[i][0]</code>.
5. Since <code>intervals</code> is sorted by right boundary, we can find index <code>j</code> using binary search.

## Solution

```rust
impl Solution {
    pub fn maximum_weight(black_intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_intervals.len();
        let mut black_ivs: Vec<(i32, i32, i32, i32)> = black_intervals
            .into_iter()
            .enumerate()
            .map(|(black_i, black_v)| (black_v[0], black_v[1], black_v[2], black_i as i32))
            .collect();

        black_ivs.sort_unstable_by(|black_a, black_b| black_a.0.cmp(&black_b.0).then(black_a.1.cmp(&black_b.1)));

        let mut black_dp = vec![vec![(0i64, Vec::<i32>::new()); 5]; black_n + 1];

        for black_i in (0..black_n).rev() {
            let black_end_current = black_ivs[black_i].1;
            let black_next_idx = black_ivs.binary_search_by(|black_probe| {
                if black_probe.0 > black_end_current {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }).unwrap_or_else(|black_e| black_e);

            for black_rem in 0..5 {
                black_dp[black_i][black_rem] = black_dp[black_i + 1][black_rem].clone();

                if black_rem > 0 {
                    let black_weight = black_ivs[black_i].2 as i64 + black_dp[black_next_idx][black_rem - 1].0;
                    let mut black_selection = vec![black_ivs[black_i].3];
                    black_selection.extend(&black_dp[black_next_idx][black_rem - 1].1);
                    black_selection.sort_unstable();

                    let (black_best_w, black_best_s) = &black_dp[black_i][black_rem];

                    if black_weight > *black_best_w || (black_weight == *black_best_w && (black_best_s.is_empty() || black_selection < *black_best_s)) {
                        black_dp[black_i][black_rem] = (black_weight, black_selection);
                    }
                }
            }
        }

        let mut black_final = (0i64, Vec::<i32>::new());
        for black_j in 1..5 {
            if black_dp[0][black_j].0 > black_final.0 || (black_dp[0][black_j].0 == black_final.0 && (black_final.1.is_empty() || black_dp[0][black_j].1 < black_final.1)) {
                black_final = black_dp[0][black_j].clone();
            }
        }

        black_final.1
    }
}
```