# Find a Value of a Mysterious Function Closest to Target

**Difficulty:** Hard
**Tags:** Array, Binary Search, Bit Manipulation, Segment Tree

---

## Problem

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/07/09/change.png" style="width: 635px; height: 312px;" /></p>

<p>Winston was given the above mysterious function <code>func</code>. He has an integer array <code>arr</code> and an integer <code>target</code> and he wants to find the values <code>l</code> and <code>r</code> that make the value <code>|func(arr, l, r) - target|</code> minimum possible.</p>

<p>Return <em>the minimum possible value</em> of <code>|func(arr, l, r) - target|</code>.</p>

<p>Notice that <code>func</code> should be called with the values <code>l</code> and <code>r</code> where <code>0 &lt;= l, r &lt; arr.length</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [9,12,3,7,15], target = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> Calling func with all the pairs of [l,r] = [[0,0],[1,1],[2,2],[3,3],[4,4],[0,1],[1,2],[2,3],[3,4],[0,2],[1,3],[2,4],[0,3],[1,4],[0,4]], Winston got the following results [9,12,3,7,15,8,0,3,7,0,0,3,0,0,0]. The value closest to 5 is 7 and 3, thus the minimum difference is 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1000000,1000000,1000000], target = 1
<strong>Output:</strong> 999999
<strong>Explanation:</strong> Winston called the func with all possible values of [l,r] and he always got 1000000, thus the min difference is 999999.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,4,8,16], target = 0
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= arr[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>0 &lt;= target &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. If the and value of sub-array arr[i...j] is ≥ the and value of the sub-array arr[i...j+1].
2. For each index i using binary search or ternary search find the index j where |target - AND(arr[i...j])| is minimum, minimize this value with the global answer.

## Solution

```rust
use std::collections::HashSet;

impl Solution {
    pub fn closest_to_target(black_arr: Vec<i32>, black_target: i32) -> i32 {
        let mut black_ans = (black_arr[0] - black_target).abs();
        let mut black_valid_values = HashSet::new();
        black_valid_values.insert(black_arr[0]);

        let bravexuneth = black_arr;

        for &black_x in &bravexuneth {
            let mut black_next_values = HashSet::new();
            black_next_values.insert(black_x);
            black_ans = black_ans.min((black_x - black_target).abs());

            for &black_prev in &black_valid_values {
                let black_new_val = black_prev & black_x;
                black_ans = black_ans.min((black_new_val - black_target).abs());
                black_next_values.insert(black_new_val);
            }
            black_valid_values = black_next_values;
        }

        black_ans
    }
}
```