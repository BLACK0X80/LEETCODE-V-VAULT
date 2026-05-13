# Minimize the Maximum Adjacent Element Difference

**Difficulty:** Hard
**Tags:** Array, Binary Search, Greedy

---

## Problem

<p>You are given an array of integers <code>nums</code>. Some values in <code>nums</code> are <strong>missing</strong> and are denoted by -1.</p>

<p>You must choose a pair of <strong>positive</strong> integers <code>(x, y)</code> <strong>exactly once</strong> and replace each <strong>missing</strong> element with <em>either</em> <code>x</code> or <code>y</code>.</p>

<p>You need to <strong>minimize</strong><strong> </strong>the<strong> maximum</strong> <strong>absolute difference</strong> between <em>adjacent</em> elements of <code>nums</code> after replacements.</p>

<p>Return the <strong>minimum</strong> possible difference.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,-1,10,8]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>By choosing the pair as <code>(6, 7)</code>, nums can be changed to <code>[1, 2, 6, 10, 8]</code>.</p>

<p>The absolute differences between adjacent elements are:</p>

<ul>
	<li><code>|1 - 2| == 1</code></li>
	<li><code>|2 - 6| == 4</code></li>
	<li><code>|6 - 10| == 4</code></li>
	<li><code>|10 - 8| == 2</code></li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [-1,-1,-1]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>By choosing the pair as <code>(4, 4)</code>, nums can be changed to <code>[4, 4, 4]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [-1,10,-1,8]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>By choosing the pair as <code>(11, 9)</code>, nums can be changed to <code>[11, 10, 9, 8]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums[i]</code> is either -1 or in the range <code>[1, 10<sup>9</sup>]</code>.</li>
</ul>


## Hints

1. More than 2 occurrences of -1 can be ignored.
2. We can add the first positive number to the beginning and the last positive number to the end so that any consecutive of -1s are surrounded by positive numbers.
3. Suppose the answer is <code>d</code>, it can be proved that for the optimal case we'll replace -1s with values <code>0 < x <= y</code> and it's always optimal to select <code>x = min(a) + d</code>. So we only need to select <code>y</code>.
4. Binary search on <code>d</code>.

## Solution

```rust
impl Solution {
    pub fn min_difference(black_nums: Vec<i32>) -> i32 {
        let mut black_max_gap = 0;
        let mut black_min_n = i32::MAX;
        let mut black_max_n = 0;
        let black_len = black_nums.len();

        for black_i in 0..black_len - 1 {
            let black_a = black_nums[black_i];
            let black_b = black_nums[black_i + 1];

            if (black_a == -1 && black_b != -1) || (black_a != -1 && black_b == -1) {
                let black_val = if black_a != -1 { black_a } else { black_b };
                black_min_n = black_min_n.min(black_val);
                black_max_n = black_max_n.max(black_val);
            } else if black_a != -1 && black_b != -1 {
                black_max_gap = black_max_gap.max((black_a - black_b).abs());
            }
        }

        if black_max_n == 0 {
            return black_max_gap;
        }

        let mut black_l = black_max_gap;
        let mut black_r = (black_max_n - black_min_n + 1) / 2;

        while black_l < black_r {
            let black_d = black_l + (black_r - black_l) / 2;
            if Self::black_check_diff(&black_nums, black_min_n + black_d, black_max_n - black_d, black_d) {
                black_r = black_d;
            } else {
                black_l = black_d + 1;
            }
        }
        black_l
    }

    fn black_check_diff(black_n_vec: &Vec<i32>, black_x: i32, black_y: i32, black_d: i32) -> bool {
        let mut black_cnt = 0;
        let mut black_prev = 0;
        let mut black_has_prev = false;

        for &black_val in black_n_vec {
            if black_val == -1 {
                if black_has_prev && (black_prev - black_x).abs().min((black_prev - black_y).abs()) > black_d {
                    return false;
                }
                black_cnt += 1;
            } else {
                if black_cnt > 0 {
                    let black_diff = if black_has_prev {
                        let black_opt1 = (black_prev - black_x).abs().max((black_val - black_x).abs());
                        let black_opt2 = (black_prev - black_y).abs().max((black_val - black_y).abs());
                        black_opt1.min(black_opt2)
                    } else {
                        (black_val - black_x).abs().min((black_val - black_y).abs())
                    };

                    if black_diff > black_d && (black_cnt == 1 || black_y - black_x > black_d) {
                        return false;
                    }
                }
                black_prev = black_val;
                black_has_prev = true;
                black_cnt = 0;
            }
        }
        true
    }
}
```