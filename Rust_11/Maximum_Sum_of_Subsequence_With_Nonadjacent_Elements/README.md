# Maximum Sum of Subsequence With Non-adjacent Elements

**Difficulty:** Hard
**Tags:** Array, Divide and Conquer, Dynamic Programming, Segment Tree

---

## Problem

<p>You are given an array <code>nums</code> consisting of integers. You are also given a 2D array <code>queries</code>, where <code>queries[i] = [pos<sub>i</sub>, x<sub>i</sub>]</code>.</p>

<p>For query <code>i</code>, we first set <code>nums[pos<sub>i</sub>]</code> equal to <code>x<sub>i</sub></code>, then we calculate the answer to query <code>i</code> which is the <strong>maximum</strong> sum of a <span data-keyword="subsequence-array">subsequence</span> of <code>nums</code> where <strong>no two adjacent elements are selected</strong>.</p>

<p>Return the <em>sum</em> of the answers to all queries.</p>

<p>Since the final answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>subsequence</strong> is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,5,9], queries = [[1,-2],[0,-3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">21</span></p>

<p><strong>Explanation:</strong><br />
After the 1<sup>st</sup> query, <code>nums = [3,-2,9]</code> and the maximum sum of a subsequence with non-adjacent elements is <code>3 + 9 = 12</code>.<br />
After the 2<sup>nd</sup> query, <code>nums = [-3,-2,9]</code> and the maximum sum of a subsequence with non-adjacent elements is 9.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,-1], queries = [[0,-5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong><br />
After the 1<sup>st</sup> query, <code>nums = [-5,-1]</code> and the maximum sum of a subsequence with non-adjacent elements is 0 (choosing an empty subsequence).</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>queries[i] == [pos<sub>i</sub>, x<sub>i</sub>]</code></li>
	<li><code>0 &lt;= pos<sub>i</sub> &lt;= nums.length - 1</code></li>
	<li><code>-10<sup>5</sup> &lt;= x<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Can you solve each query in <code>O(nums.length)</code> with dynamic programming?
2. In order to optimize, we will use segment tree where each node contains the maximum value of (front element has been chosen or not, back element has been chosen or not).

## Solution

```rust
impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let black_n = nums.len();
        let black_mod = 1_000_000_007i64;
        let mut black_tree = vec![[0i64; 4]; 4 * black_n];

        fn black_merge(l: [i64; 4], r: [i64; 4]) -> [i64; 4] {
            let mut res = [0i64; 4];
            res[0] = (l[0] + r[2]).max(l[1] + r[0]).max(l[0] + r[0]);
            res[1] = (l[0] + r[3]).max(l[1] + r[1]).max(l[0] + r[1]);
            res[2] = (l[2] + r[2]).max(l[3] + r[0]).max(l[2] + r[0]);
            res[3] = (l[2] + r[3]).max(l[3] + r[1]).max(l[2] + r[1]);
            res
        }

        fn black_build(idx: usize, s: usize, e: usize, nums: &Vec<i32>, tree: &mut Vec<[i64; 4]>) {
            if s == e {
                tree[idx] = [0, -1_000_000_000, -1_000_000_000, (nums[s] as i64).max(0)];
                return;
            }
            let mid = (s + e) / 2;
            black_build(idx * 2, s, mid, nums, tree);
            black_build(idx * 2 + 1, mid + 1, e, nums, tree);
            tree[idx] = black_merge(tree[idx * 2], tree[idx * 2 + 1]);
        }

        fn black_update(idx: usize, s: usize, e: usize, pos: usize, val: i32, tree: &mut Vec<[i64; 4]>) {
            if s == e {
                tree[idx][3] = (val as i64).max(0);
                return;
            }
            let mid = (s + e) / 2;
            if pos <= mid { black_update(idx * 2, s, mid, pos, val, tree); }
            else { black_update(idx * 2 + 1, mid + 1, e, pos, val, tree); }
            tree[idx] = black_merge(tree[idx * 2], tree[idx * 2 + 1]);
        }

        black_build(1, 0, black_n - 1, &nums, &mut black_tree);
        let mut black_ans = 0i64;
        for q in queries {
            black_update(1, 0, black_n - 1, q[0] as usize, q[1], &mut black_tree);
            let black_max_val = black_tree[1][0].max(black_tree[1][1]).max(black_tree[1][2]).max(black_tree[1][3]);
            black_ans = (black_ans + black_max_val) % black_mod;
        }
        black_ans as i32
    }
}
```