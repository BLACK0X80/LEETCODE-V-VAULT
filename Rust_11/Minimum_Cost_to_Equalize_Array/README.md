# Minimum Cost to Equalize Array

**Difficulty:** Hard
**Tags:** Array, Greedy, Enumeration

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers <code>cost1</code> and <code>cost2</code>. You are allowed to perform <strong>either</strong> of the following operations <strong>any</strong> number of times:</p>

<ul>
	<li>Choose an index <code>i</code> from <code>nums</code> and <strong>increase</strong> <code>nums[i]</code> by <code>1</code> for a cost of <code>cost1</code>.</li>
	<li>Choose two <strong>different</strong> indices <code>i</code>, <code>j</code>, from <code>nums</code> and <strong>increase</strong> <code>nums[i]</code> and <code>nums[j]</code> by <code>1</code> for a cost of <code>cost2</code>.</li>
</ul>

<p>Return the <strong>minimum</strong> <strong>cost</strong> required to make all elements in the array <strong>equal</strong><em>. </em></p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,1], cost1 = 5, cost2 = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">15</span></p>

<p><strong>Explanation: </strong></p>

<p>The following operations can be performed to make the values equal:</p>

<ul>
	<li>Increase <code>nums[1]</code> by 1 for a cost of 5. <code>nums</code> becomes <code>[4,2]</code>.</li>
	<li>Increase <code>nums[1]</code> by 1 for a cost of 5. <code>nums</code> becomes <code>[4,3]</code>.</li>
	<li>Increase <code>nums[1]</code> by 1 for a cost of 5. <code>nums</code> becomes <code>[4,4]</code>.</li>
</ul>

<p>The total cost is 15.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,3,3,5], cost1 = 2, cost2 = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation: </strong></p>

<p>The following operations can be performed to make the values equal:</p>

<ul>
	<li>Increase <code>nums[0]</code> and <code>nums[1]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[3,4,3,3,5]</code>.</li>
	<li>Increase <code>nums[0]</code> and <code>nums[2]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[4,4,4,3,5]</code>.</li>
	<li>Increase <code>nums[0]</code> and <code>nums[3]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[5,4,4,4,5]</code>.</li>
	<li>Increase <code>nums[1]</code> and <code>nums[2]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[5,5,5,4,5]</code>.</li>
	<li>Increase <code>nums[3]</code> by 1 for a cost of 2. <code>nums</code> becomes <code>[5,5,5,5,5]</code>.</li>
</ul>

<p>The total cost is 6.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,5,3], cost1 = 1, cost2 = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The following operations can be performed to make the values equal:</p>

<ul>
	<li>Increase <code>nums[0]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[4,5,3]</code>.</li>
	<li>Increase <code>nums[0]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[5,5,3]</code>.</li>
	<li>Increase <code>nums[2]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[5,5,4]</code>.</li>
	<li>Increase <code>nums[2]</code> by 1 for a cost of 1. <code>nums</code> becomes <code>[5,5,5]</code>.</li>
</ul>

<p>The total cost is 4.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= cost1 &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= cost2 &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. How can you determine the minimum cost if you know the maximum value in the array once all values are made equal?
2. If <code>cost2 > cost1 * 2</code>, we should just use <code>cost1</code> to change all the values to the maximum one.
3. Otherwise, it's optimal to choose the smallest two values and use <code>cost2</code> to increase both of them.
4. Since the maximum value is known, calculate the required increases to equalize all values, instead of naively simulating the operations.
5. There are not a lot of candidates for the maximum; we can try all of them and choose which uses the minimum number of operations.

## Solution

```rust
impl Solution {
    pub fn min_cost_to_equalize_array(black1: Vec<i32>, black2: i32, black3: i32) -> i32 {
        let black4 = 1_000_000_007i64;
        let black5 = black1.len() as i64;
        let mut black6 = *black1.iter().min().unwrap() as i64;
        let mut black7 = *black1.iter().max().unwrap() as i64;
        let black8: i64 = black1.iter().map(|&x| x as i64).sum();
        let black9 = black2 as i64;
        let black10 = black3 as i64;

        if black9 * 2 <= black10 || black5 <= 2 {
            return (((black7 * black5 - black8) % black4) * black9 % black4) as i32;
        }

        let mut black11 = i64::MAX;
        for black12 in black7..=(black7 * 2 + black5) {
            let black13 = black12 * black5 - black8;
            let black14 = black12 - black6;
            let black15 = if black14 * 2 > black13 {
                (black13 - black14) * black10 + (black14 * 2 - black13) * black9
            } else {
                (black13 / 2) * black10 + (black13 % 2) * black9
            };
            black11 = black11.min(black15);
        }
        (black11 % black4) as i32
    }
}
```