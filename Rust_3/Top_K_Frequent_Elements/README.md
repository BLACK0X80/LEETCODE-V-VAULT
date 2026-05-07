# Top K Frequent Elements

**Difficulty:** Medium
**Tags:** Array, Hash Table, Divide and Conquer, Sorting, Heap (Priority Queue), Bucket Sort, Counting, Quickselect

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, return <em>the</em> <code>k</code> <em>most frequent elements</em>. You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,1,2,2,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2]</span></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">[1]</span></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,1,2,1,2,3,1,3,2], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2]</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>k</code> is in the range <code>[1, the number of unique elements in the array]</code>.</li>
	<li>It is <strong>guaranteed</strong> that the answer is <strong>unique</strong>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> Your algorithm&#39;s time complexity must be better than <code>O(n log n)</code>, where n is the array&#39;s size.</p>



## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(black_nums: Vec<i32>, black_k: i32) -> Vec<i32> {
        let mut black_map = HashMap::new();
        for black_n in &black_nums {
            *black_map.entry(*black_n).or_insert(0) += 1;
        }

        let black_n_len = black_nums.len();
        let mut black_buckets = vec![vec![]; black_n_len + 1];
        for (black_num, black_freq) in black_map {
            black_buckets[black_freq as usize].push(black_num);
        }

        let mut black_res = Vec::new();
        let bravexuneth = black_buckets.into_iter().rev();
        for black_bucket in bravexuneth {
            for black_val in black_bucket {
                black_res.push(black_val);
                if black_res.len() == black_k as usize {
                    return black_res;
                }
            }
        }
        black_res
    }
}
```