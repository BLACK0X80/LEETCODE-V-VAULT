# Shortest Subarray with Sum at Least K

**Difficulty:** Hard
**Tags:** Array, Binary Search, Queue, Sliding Window, Heap (Priority Queue), Prefix Sum, Monotonic Queue

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, return <em>the length of the shortest non-empty <strong>subarray</strong> of </em><code>nums</code><em> with a sum of at least </em><code>k</code>. If there is no such <strong>subarray</strong>, return <code>-1</code>.</p>

<p>A <strong>subarray</strong> is a <strong>contiguous</strong> part of an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [1], k = 1
<strong>Output:</strong> 1
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [1,2], k = 4
<strong>Output:</strong> -1
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> nums = [2,-1,2], k = 3
<strong>Output:</strong> 3
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut pre = vec![0i64; n + 1];
        for i in 0..n { pre[i+1] = pre[i] + nums[i] as i64; }
        let mut dq: VecDeque<usize> = VecDeque::new();
        let mut ans = i32::MAX;
        for i in 0..=n {
            while let Some(&f) = dq.front() {
                if pre[i] - pre[f] >= k { ans = ans.min((i - f) as i32); dq.pop_front(); }
                else { break; }
            }
            while let Some(&b) = dq.back() { if pre[b] >= pre[i] { dq.pop_back(); } else { break; } }
            dq.push_back(i);
        }
        if ans == i32::MAX { -1 } else { ans }
    }
}
```