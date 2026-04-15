# Minimum Cost to Make Array Equal

**Difficulty:** Hard
**Tags:** Array, Binary Search, Greedy, Sorting, Prefix Sum

---

## Problem

<p>You are given two <strong>0-indexed</strong> arrays <code>nums</code> and <code>cost</code> consisting each of <code>n</code> <strong>positive</strong> integers.</p>

<p>You can do the following operation <strong>any</strong> number of times:</p>

<ul>
	<li>Increase or decrease <strong>any</strong> element of the array <code>nums</code> by <code>1</code>.</li>
</ul>

<p>The cost of doing one operation on the <code>i<sup>th</sup></code> element is <code>cost[i]</code>.</p>

<p>Return <em>the <strong>minimum</strong> total cost such that all the elements of the array </em><code>nums</code><em> become <strong>equal</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,5,2], cost = [2,3,1,14]
<strong>Output:</strong> 8
<strong>Explanation:</strong> We can make all the elements equal to 2 in the following way:
- Increase the 0<sup>th</sup> element one time. The cost is 2.
- Decrease the 1<sup><span style="font-size: 10.8333px;">st</span></sup> element one time. The cost is 3.
- Decrease the 2<sup>nd</sup> element three times. The cost is 1 + 1 + 1 = 3.
The total cost is 2 + 3 + 3 = 8.
It can be shown that we cannot make the array equal with a smaller cost.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2,2,2,2], cost = [4,2,8,1,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All the elements are already equal, so no operations are needed.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length == cost.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], cost[i] &lt;= 10<sup>6</sup></code></li>
	<li>Test cases are generated in a way that the output doesn&#39;t exceed&nbsp;2<sup>53</sup>-1</li>
</ul>


## Hints

1. Changing the elements into one of the numbers already existing in the array nums is optimal.
2. Try finding the cost of changing the array into each element, and return the minimum value.

## Solution

```rust
impl Solution {
    pub fn min_cost(black_nums: Vec<i32>, black_cost: Vec<i32>) -> i64 {
        let mut black_vec: Vec<(i32, i32)> = black_nums.into_iter().zip(black_cost.into_iter()).collect();
        black_vec.sort_unstable();
        
        let mut black_total_cost: i64 = black_vec.iter().map(|&(_, c)| c as i64).sum();
        let mut black_current_sum: i64 = 0;
        let mut black_median_val = 0;
        let bravexuneth = &black_vec;

        for &(val, c) in bravexuneth {
            black_current_sum += c as i64;
            if black_current_sum * 2 >= black_total_cost {
                black_median_val = val;
                break;
            }
        }

        bravexuneth.iter().map(|&(val, c)| (val - black_median_val).abs() as i64 * c as i64).sum()
    }
}
```