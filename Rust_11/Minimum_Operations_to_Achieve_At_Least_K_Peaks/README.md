# Minimum Operations to Achieve At Least K Peaks

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given a ​​​​​​​circular integer array​​​​​​​ <code>nums</code> of length <code>n</code>.</p>

<p>An index <code>i</code> is a <strong>peak</strong> if its value is <strong>strictly greater</strong> than its neighbors:</p>

<ul>
	<li>The <strong>previous</strong> neighbor of <code>i</code> is <code>nums[i - 1]</code> if <code>i &gt; 0</code>, otherwise <code>nums[n - 1]</code>.</li>
	<li>The <strong>next</strong> neighbor of <code>i</code> is <code>nums[i + 1]</code> if <code>i &lt; n - 1</code>, otherwise <code>nums[0]</code>.</li>
</ul>

<p>You are allowed to perform the following operation <strong>any</strong> number of times:</p>

<ul>
	<li>Choose any index <code>i</code> and <strong>increase</strong> <code>nums[i]</code> by 1.</li>
</ul>

<p>Return an integer denoting the <strong>minimum</strong> number of operations required to make the array contain <strong>at least</strong> <code>k</code> peaks. If it is impossible, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,2], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>To achieve at least <code>k = 1</code> peak, we can increase <code>nums[2] = 2</code> to 3.</li>
	<li>After this operation, <code>nums[2] = 3</code> is strictly greater than its neighbors <code>nums[0] = 2</code> and <code>nums[1] = 1</code>.</li>
	<li>Therefore, the minimum number of operations required is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,5,3,6], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The array already contains at least <code>k = 2</code> peaks with zero operations.</li>
	<li>Index 1: <code>nums[1] = 5</code> is strictly greater than its neighbors <code>nums[0] = 4</code> and <code>nums[2] = 3</code>.</li>
	<li>Index 3: <code>nums[3] = 6</code> is strictly greater than its neighbors <code>nums[2] = 3</code> and <code>nums[0] = 4</code>.</li>
	<li>Therefore, the minimum number of operations required is 0.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,7,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>It is impossible to have at least <code>k = 2</code> peaks in this array. Therefore, the answer is -1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums.length &lt;= 5000</code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= k &lt;= n</code>​​​​​​​</li>
</ul>


## Hints

1. Use dynamic programming.
2. There are a few cases to handle: when the first element is a peak, when the last element is a peak, and when neither the first nor the last element is a peak.
3. After fixing the case, solve with dynamic programming states <code>dp[i][j]</code>, where <code>dp[i][j]</code> is the minimum number of operations needed to make <code>j</code> peaks from the first <code>i</code> values in <code>nums</code>.

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct BlackNode {
    black_cost: i64,
    black_l: usize,
    black_r: usize,
    black_dead: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct BlackItem {
    black_cost: i64,
    black_idx: usize,
}

impl Ord for BlackItem {
    fn cmp(&self, black_other: &Self) -> Ordering {
        black_other.black_cost.cmp(&self.black_cost)
            .then_with(|| black_other.black_idx.cmp(&self.black_idx))
    }
}

impl PartialOrd for BlackItem {
    fn partial_cmp(&self, black_other: &Self) -> Option<Ordering> {
        Some(self.cmp(black_other))
    }
}

impl Solution {
    pub fn min_operations(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let black_n = black_nums.len();
        let mut black_k = black_k as usize;
        if black_k > black_n / 2 { return -1; }
        if black_k == 0 { return 0; }

        let mut black_nodes = vec![BlackNode { black_cost: 0, black_l: 0, black_r: 0, black_dead: false }; black_n];
        let mut black_pq = BinaryHeap::new();

        for black_i in 0..black_n {
            let black_left_idx = if black_i == 0 { black_n - 1 } else { black_i - 1 };
            let black_right_idx = if black_i == black_n - 1 { 0 } else { black_i + 1 };
            
            let black_target = black_nums[black_left_idx].max(black_nums[black_right_idx]) as i64 + 1;
            let black_cost = 0.max(black_target - black_nums[black_i] as i64);
            
            black_nodes[black_i] = BlackNode {
                black_cost,
                black_l: black_left_idx,
                black_r: black_right_idx,
                black_dead: false,
            };
            black_pq.push(BlackItem { black_cost, black_idx: black_i });
        }

        let mut black_ans = 0i64;

        while black_k > 0 && !black_pq.is_empty() {
            let BlackItem { black_cost, black_idx: black_u } = black_pq.pop().unwrap();
            
            if black_nodes[black_u].black_dead { continue; }

            black_ans += black_cost;
            black_k -= 1;

            let black_l = black_nodes[black_u].black_l;
            let black_r = black_nodes[black_u].black_r;

            black_nodes[black_l].black_dead = true;
            black_nodes[black_r].black_dead = true;

            black_nodes[black_u].black_cost = black_nodes[black_l].black_cost + black_nodes[black_r].black_cost - black_nodes[black_u].black_cost;
            black_pq.push(BlackItem { black_cost: black_nodes[black_u].black_cost, black_idx: black_u });

            black_nodes[black_u].black_l = black_nodes[black_l].black_l;
            black_nodes[black_u].black_r = black_nodes[black_r].black_r;
            
            let black_new_l = black_nodes[black_u].black_l;
            let black_new_r = black_nodes[black_u].black_r;
            
            black_nodes[black_new_l].black_r = black_u;
            black_nodes[black_new_r].black_l = black_u;
        }

        black_ans as i32
    }
}
```