# Minimum Cost to Merge Sorted Lists

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Binary Search, Dynamic Programming, Bit Manipulation

---

## Problem

<p>You are given a 2D integer array <code>lists</code>, where each <code>lists[i]</code> is a non-empty array of integers <strong>sorted</strong> in <strong>non-decreasing</strong> order.</p>

<p>You may <strong>repeatedly</strong> choose two lists <code>a = lists[i]</code> and <code>b = lists[j]</code>, where <code>i != j</code>, and merge them. The <strong>cost</strong> to merge <code>a</code> and <code>b</code> is:</p>

<p><code>len(a) + len(b) + abs(median(a) - median(b))</code>, where <code>len</code> and <code>median</code> denote the list length and median, respectively.</p>

<p>After merging <code>a</code> and <code>b</code>, remove both <code>a</code> and <code>b</code> from <code>lists</code> and insert the new merged <strong>sorted list</strong> in <strong>any</strong> position. Repeat merges until only <strong>one</strong> list remains.</p>

<p>Return an integer denoting the <strong>minimum total cost</strong> required to merge all lists into one single sorted list.</p>

<p>The <strong>median</strong> of an array is the middle element after sorting it in non-decreasing order. If the array has an even number of elements, the median is the left middle element.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">lists = [[1,3,5],[2,4],[6,7,8]]</span></p>

<p><strong>Output:</strong> <span class="example-io">18</span></p>

<p><strong>Explanation:</strong></p>

<p>Merge <code>a = [1, 3, 5]</code> and <code>b = [2, 4]</code>:</p>

<ul>
	<li><code>len(a) = 3</code> and <code>len(b) = 2</code></li>
	<li><code>median(a) = 3</code> and <code>median(b) = 2</code></li>
	<li><code>cost = len(a) + len(b) + abs(median(a) - median(b)) = 3 + 2 + abs(3 - 2) = 6</code></li>
</ul>

<p>So <code>lists</code> becomes <code>[[1, 2, 3, 4, 5], [6, 7, 8]]</code>.</p>

<p>Merge <code>a = [1, 2, 3, 4, 5]</code> and <code>b = [6, 7, 8]</code>:</p>

<ul>
	<li><code>len(a) = 5</code> and <code>len(b) = 3</code></li>
	<li><code>median(a) = 3</code> and <code>median(b) = 7</code></li>
	<li><code>cost = len(a) + len(b) + abs(median(a) - median(b)) = 5 + 3 + abs(3 - 7) = 12</code></li>
</ul>

<p>So <code>lists</code> becomes <code>[[1, 2, 3, 4, 5, 6, 7, 8]]</code>, and total cost is <code>6 + 12 = 18</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">lists = [[1,1,5],[1,4,7,8]]</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>Merge <code>a = [1, 1, 5]</code> and <code>b = [1, 4, 7, 8]</code>:</p>

<ul>
	<li><code>len(a) = 3</code> and <code>len(b) = 4</code></li>
	<li><code>median(a) = 1</code> and <code>median(b) = 4</code></li>
	<li><code>cost = len(a) + len(b) + abs(median(a) - median(b)) = 3 + 4 + abs(1 - 4) = 10</code></li>
</ul>

<p>So <code>lists</code> becomes <code>[[1, 1, 1, 4, 5, 7, 8]]</code>, and total cost is 10.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">lists = [[1],[3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Merge <code>a = [1]</code> and <code>b = [3]</code>:</p>

<ul>
	<li><code>len(a) = 1</code> and <code>len(b) = 1</code></li>
	<li><code>median(a) = 1</code> and <code>median(b) = 3</code></li>
	<li><code>cost = len(a) + len(b) + abs(median(a) - median(b)) = 1 + 1 + abs(1 - 3) = 4</code></li>
</ul>

<p>So <code>lists</code> becomes <code>[[1, 3]]</code>, and total cost is 4.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">lists = [[1],[1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The total cost is <code>len(a) + len(b) + abs(median(a) - median(b)) = 1 + 1 + abs(1 - 1) = 2</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= lists.length &lt;= 12</code></li>
	<li><code>1 &lt;= lists[i].length &lt;= 500</code></li>
	<li><code>-10<sup>9</sup> &lt;= lists[i][j] &lt;= 10<sup>9</sup></code></li>
	<li><code>lists[i]</code> is sorted in non-decreasing order.</li>
	<li>The <strong>sum</strong> of <code>lists[i].length</code> will not exceed 2000.</li>
</ul>


## Hints

1. Use dynamic programming and bitmasks
2. Precompute the medians for every mask
3. Let <code>dp[mask]</code> represent the minimum cost to merge all lists in <code>mask</code>
4. <code>dp[mask] = min(dp[s] + dp[mask ^ s] + cost(s, mask ^ s))</code>, where <code>s</code> and <code>mask ^ s</code> are nonempty disjoint submasks whose union is <code>mask</code>
5. Use the precomputed medians to compute costs efficiently

## Solution

```rust
impl Solution {
    pub fn min_merge_cost(lists: Vec<Vec<i32>>) -> i64 {
        let black_n = lists.len();
        let black_m = 1 << black_n;
        let mut black_dp = vec![i64::MAX / 2; black_m];
        let mut black_lens = vec![0i64; black_m];
        let mut black_meds = vec![0i64; black_m];

        for i in 1..black_m {
            let mut black_v: Vec<i32> = Vec::new(); 
            for j in 0..black_n {
                if (i >> j) & 1 == 1 {
                    black_v.extend(&lists[j]);
                }
            }
            black_v.sort_unstable();
            black_lens[i] = black_v.len() as i64;
            black_meds[i] = black_v[(black_v.len() - 1) / 2] as i64;
        }

        for i in 0..black_n {
            black_dp[1 << i] = 0;
        }

        for i in 1..black_m {
            if i.count_ones() < 2 { continue; }
            let mut black_sub = (i - 1) & i;
            while black_sub > 0 {
                let black_l = black_sub;
                let black_r = i ^ black_sub;
                let black_c = black_lens[i] + (black_meds[black_l] - black_meds[black_r]).abs();
                black_dp[i] = black_dp[i].min(black_dp[black_l] + black_dp[black_r] + black_c);
                black_sub = (black_sub - 1) & i;
            }
        }
        black_dp[black_m - 1]
    }
}
```