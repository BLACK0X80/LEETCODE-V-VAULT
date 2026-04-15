# Find Subarray With Bitwise OR Closest to K

**Difficulty:** Hard
**Tags:** Array, Binary Search, Bit Manipulation, Segment Tree

---

## Problem

<p>You are given an array <code>nums</code> and an integer <code>k</code>. You need to find a <span data-keyword="subarray-nonempty">subarray</span> of <code>nums</code> such that the <strong>absolute difference</strong> between <code>k</code> and the bitwise <code>OR</code> of the subarray elements is as<strong> small</strong> as possible. In other words, select a subarray <code>nums[l..r]</code> such that <code>|k - (nums[l] OR nums[l + 1] ... OR nums[r])|</code> is minimum.</p>

<p>Return the <strong>minimum</strong> possible value of the absolute difference.</p>

<p>A <strong>subarray</strong> is a contiguous <b>non-empty</b> sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,4,5], k = 3</span></p>

<p><strong>Output:</strong> 0</p>

<p><strong>Explanation:</strong></p>

<p>The subarray <code>nums[0..1]</code> has <code>OR</code> value 3, which gives the minimum absolute difference <code>|3 - 3| = 0</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,1,3], k = 2</span></p>

<p><strong>Output:</strong> 1</p>

<p><strong>Explanation:</strong></p>

<p>The subarray <code>nums[1..1]</code> has <code>OR</code> value 3, which gives the minimum absolute difference <code>|3 - 2| = 1</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1], k = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<p>There is a single subarray with <code>OR</code> value 1, which gives the minimum absolute difference <code>|10 - 1| = 9</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Let <code>dp[i]</code> be the set of all the bitwise <code>OR</code> of all the subarrays ending at index <code>i</code>.
2. We start from <code>nums[i]</code>, taking the bitwise <code>OR</code> result by including elements one by one from <code>i</code> towards left. Notice that only unset bits can become set on adding an element, and set bits never become unset again.
3. Hence <code>dp[i]</code> can contain at most 30 elements.

## Solution

```rust
impl Solution {
    pub fn minimum_difference(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let (mut black_ans, mut black_set) = ((black_k - black_nums[0]).abs(), vec![black_nums[0]]);
        for black_x in black_nums {
            let mut black_next = vec![black_x];
            black_ans = black_ans.min((black_k - black_x).abs());
            for &black_v in &black_set {
                let black_new_v = black_v | black_x;
                if black_new_v != *black_next.last().unwrap() {
                    black_next.push(black_new_v);
                    black_ans = black_ans.min((black_k - black_new_v).abs());
                }
            }
            black_set = black_next;
        }
        black_ans
    }
}
```