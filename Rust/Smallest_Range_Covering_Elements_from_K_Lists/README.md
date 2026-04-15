# Smallest Range Covering Elements from K Lists

**Difficulty:** Hard
**Tags:** Array, Hash Table, Greedy, Sliding Window, Sorting, Heap (Priority Queue)

---

## Problem

<p>You have <code>k</code> lists of sorted integers in <strong>non-decreasing&nbsp;order</strong>. Find the <b>smallest</b> range that includes at least one number from each of the <code>k</code> lists.</p>

<p>We define the range <code>[a, b]</code> is smaller than range <code>[c, d]</code> if <code>b - a &lt; d - c</code> <strong>or</strong> <code>a &lt; c</code> if <code>b - a == d - c</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
<strong>Output:</strong> [20,24]
<strong>Explanation: </strong>
List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
List 2: [0, 9, 12, 20], 20 is in range [20,24].
List 3: [5, 18, 22, 30], 22 is in range [20,24].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [[1,2,3],[1,2,3],[1,2,3]]
<strong>Output:</strong> [1,1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>nums.length == k</code></li>
	<li><code>1 &lt;= k &lt;= 3500</code></li>
	<li><code>1 &lt;= nums[i].length &lt;= 50</code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i][j] &lt;= 10<sup>5</sup></code></li>
	<li><code>nums[i]</code>&nbsp;is sorted in <strong>non-decreasing</strong> order.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut indices = vec![0usize; k];
        let mut range = vec![0, i32::MAX];

        loop {
            let (mut cur_min, mut cur_max, mut min_idx) = (i32::MAX, i32::MIN, 0);
            for i in 0..k {
                let val = nums[i][indices[i]];
                if val < cur_min { cur_min = val; min_idx = i; }
                if val > cur_max { cur_max = val; }
            }
            if cur_max - cur_min < range[1] - range[0] {
                range[0] = cur_min;
                range[1] = cur_max;
            }
            indices[min_idx] += 1;
            if indices[min_idx] == nums[min_idx].len() { break; }
        }

        range
    }
}
```