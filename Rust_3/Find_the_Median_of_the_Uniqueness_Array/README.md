# Find the Median of the Uniqueness Array

**Difficulty:** Hard
**Tags:** Array, Hash Table, Binary Search, Sliding Window

---

## Problem

<p>You are given an integer array <code>nums</code>. The <strong>uniqueness array</strong> of <code>nums</code> is the sorted array that contains the number of distinct elements of all the <span data-keyword="subarray-nonempty">subarrays</span> of <code>nums</code>. In other words, it is a sorted array consisting of <code>distinct(nums[i..j])</code>, for all <code>0 &lt;= i &lt;= j &lt; nums.length</code>.</p>

<p>Here, <code>distinct(nums[i..j])</code> denotes the number of distinct elements in the subarray that starts at index <code>i</code> and ends at index <code>j</code>.</p>

<p>Return the <strong>median</strong> of the <strong>uniqueness array</strong> of <code>nums</code>.</p>

<p><strong>Note</strong> that the <strong>median</strong> of an array is defined as the middle element of the array when it is sorted in non-decreasing order. If there are two choices for a median, the <strong>smaller</strong> of the two values is taken.<!-- notionvc: 7e0f5178-4273-4a82-95ce-3395297921dc --></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The uniqueness array of <code>nums</code> is <code>[distinct(nums[0..0]), distinct(nums[1..1]), distinct(nums[2..2]), distinct(nums[0..1]), distinct(nums[1..2]), distinct(nums[0..2])]</code> which is equal to <code>[1, 1, 1, 2, 2, 3]</code>. The uniqueness array has a median of 1. Therefore, the answer is 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,4,3,4,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The uniqueness array of <code>nums</code> is <code>[1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3]</code>. The uniqueness array has a median of 2. Therefore, the answer is 2.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,3,5,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The uniqueness array of <code>nums</code> is <code>[1, 1, 1, 1, 2, 2, 2, 3, 3, 3]</code>. The uniqueness array has a median of 2. Therefore, the answer is 2.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Binary search over the answer.
2. For a given <code>x</code>, you need to check if <code>x</code> is the median, to the left of the median, or to the right of the median. You can do that by counting the number of sub-arrays <code>nums[i…j]</code> such that <code>distinct(num[i…j]) <= x</code>.
3. Use the sliding window to solve the counting problem in the hint above.

## Solution

```rust
impl Solution {
    pub fn median_of_uniqueness_array(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let black_total = black_n as i64 * (black_n as i64 + 1) / 2;
        let black_check = |black_m: i32| {
            let (mut black_l, mut black_count, mut black_map) = (0, 0i64, std::collections::HashMap::new());
            for black_r in 0..black_n {
                *black_map.entry(black_nums[black_r]).or_insert(0) += 1;
                while black_map.len() > black_m as usize {
                    let black_v = black_map.get_mut(&black_nums[black_l]).unwrap(); *black_v -= 1;
                    if *black_v == 0 { black_map.remove(&black_nums[black_l]); }
                    black_l += 1;
                }
                black_count += (black_r - black_l + 1) as i64;
            }
            black_count >= (black_total + 1) / 2
        };
        let (mut black_low, mut black_high, mut black_res) = (1, black_n as i32, 1);
        while black_low <= black_high {
            let black_mid = (black_low + black_high) / 2;
            if black_check(black_mid) { black_res = black_mid; black_high = black_mid - 1; } else { black_low = black_mid + 1; }
        }
        black_res
    }
}
```