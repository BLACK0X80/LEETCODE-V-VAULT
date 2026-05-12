# Count the Number of Houses at a Certain Distance II

**Difficulty:** Hard
**Tags:** Graph Theory, Prefix Sum

---

## Problem

<p>You are given three <strong>positive</strong> integers <code>n</code>, <code>x</code>, and <code>y</code>.</p>

<p>In a city, there exist houses numbered <code>1</code> to <code>n</code> connected by <code>n</code> streets. There is a street connecting the house numbered <code>i</code> with the house numbered <code>i + 1</code> for all <code>1 &lt;= i &lt;= n - 1</code> . An additional street connects the house numbered <code>x</code> with the house numbered <code>y</code>.</p>

<p>For each <code>k</code>, such that <code>1 &lt;= k &lt;= n</code>, you need to find the number of <strong>pairs of houses</strong> <code>(house<sub>1</sub>, house<sub>2</sub>)</code> such that the <strong>minimum</strong> number of streets that need to be traveled to reach <code>house<sub>2</sub></code> from <code>house<sub>1</sub></code> is <code>k</code>.</p>

<p>Return <em>a <strong>1-indexed</strong> array </em><code>result</code><em> of length </em><code>n</code><em> where </em><code>result[k]</code><em> represents the <strong>total</strong> number of pairs of houses such that the <strong>minimum</strong> streets required to reach one house from the other is </em><code>k</code>.</p>

<p><strong>Note</strong> that <code>x</code> and <code>y</code> can be <strong>equal</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/12/20/example2.png" style="width: 474px; height: 197px;" />
<pre>
<strong>Input:</strong> n = 3, x = 1, y = 3
<strong>Output:</strong> [6,0,0]
<strong>Explanation:</strong> Let&#39;s look at each pair of houses:
- For the pair (1, 2), we can go from house 1 to house 2 directly.
- For the pair (2, 1), we can go from house 2 to house 1 directly.
- For the pair (1, 3), we can go from house 1 to house 3 directly.
- For the pair (3, 1), we can go from house 3 to house 1 directly.
- For the pair (2, 3), we can go from house 2 to house 3 directly.
- For the pair (3, 2), we can go from house 3 to house 2 directly.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/12/20/example3.png" style="width: 668px; height: 174px;" />
<pre>
<strong>Input:</strong> n = 5, x = 2, y = 4
<strong>Output:</strong> [10,8,2,0,0]
<strong>Explanation:</strong> For each distance k the pairs are:
- For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (2, 4), (4, 2), (3, 4), (4, 3), (4, 5), and (5, 4).
- For k == 2, the pairs are (1, 3), (3, 1), (1, 4), (4, 1), (2, 5), (5, 2), (3, 5), and (5, 3).
- For k == 3, the pairs are (1, 5), and (5, 1).
- For k == 4 and k == 5, there are no pairs.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/12/20/example5.png" style="width: 544px; height: 130px;" />
<pre>
<strong>Input:</strong> n = 4, x = 1, y = 1
<strong>Output:</strong> [6,4,2,0]
<strong>Explanation:</strong> For each distance k the pairs are:
- For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (3, 4), and (4, 3).
- For k == 2, the pairs are (1, 3), (3, 1), (2, 4), and (4, 2).
- For k == 3, the pairs are (1, 4), and (4, 1).
- For k == 4, there are no pairs.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= x, y &lt;= n</code></li>
</ul>


## Hints

1. If there were no additional street connecting house <code>x</code> to house <code>y</code>, there would be <code>2 * (n - i)</code> pairs of houses at distance <code>i</code>.
2. The shortest distance between house <code>i</code> and house <code>j</code> (<code>j < i</code>) is along one of these paths:
- <code>i -> j</code>
- <code>i -> y---x -> j</code>
3. Try to change the distances calculated by path <code>i ->j</code> to the other path.
4. Can we use prefix sums to compute the answer?

## Solution

```rust
impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let mut black_x = x as i64;
        let mut black_y = y as i64;
        if black_x > black_y {
            std::mem::swap(&mut black_x, &mut black_y);
        }

        let black_n_idx = n as usize;
        let mut black_a = vec![0i64; black_n_idx];

        for black_i in 1..=n as i64 {
            black_a[0] += 2;

            let black_dist_1 = (black_i - 1).min((black_i - black_y).abs() + black_x);
            if black_dist_1 < n as i64 {
                black_a[black_dist_1 as usize] -= 1;
            }

            let black_dist_n = (n as i64 - black_i).min((black_i - black_x).abs() + 1 + n as i64 - black_y);
            if black_dist_n < n as i64 {
                black_a[black_dist_n as usize] -= 1;
            }

            let black_split_x = (black_i - black_x).abs().min((black_y - black_i).abs() + 1);
            if black_split_x < n as i64 {
                black_a[black_split_x as usize] += 1;
            }

            let black_split_y = ((black_i - black_x).abs() + 1).min((black_y - black_i).abs());
            if black_split_y < n as i64 {
                black_a[black_split_y as usize] += 1;
            }

            let black_r = (black_x - black_i).max(0) + (black_i - black_y).max(0);
            
            let black_mid1 = black_r + (black_y - black_x + 0) / 2;
            if black_mid1 < n as i64 {
                black_a[black_mid1 as usize] -= 1;
            }

            let black_mid2 = black_r + (black_y - black_x + 1) / 2;
            if black_mid2 < n as i64 {
                black_a[black_mid2 as usize] -= 1;
            }
        }

        for black_idx in 1..black_n_idx {
            black_a[black_idx] += black_a[black_idx - 1];
        }

        black_a
    }
}
```