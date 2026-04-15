# Maximum Equal Frequency

**Difficulty:** Hard
**Tags:** Array, Hash Table

---

## Problem

<p>Given an array <code>nums</code> of positive integers, return the longest possible length of an array prefix of <code>nums</code>, such that it is possible to remove <strong>exactly one</strong> element from this prefix so that every number that has appeared in it will have the same number of occurrences.</p>

<p>If after removing one element there are no remaining elements, it&#39;s still considered that every appeared number has the same number of ocurrences (0).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2,1,1,5,3,3,5]
<strong>Output:</strong> 7
<strong>Explanation:</strong> For the subarray [2,2,1,1,5,3,3] of length 7, if we remove nums[4] = 5, we will get [2,2,1,1,3,3], so that each number will appear exactly twice.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1,2,2,2,3,3,3,4,4,4,5]
<strong>Output:</strong> 13
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Keep track of the min and max frequencies.
2. The number to be eliminated must have a frequency of 1, same as the others or the same +1.

## Solution

```rust
impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let (mut black_c, mut black_f, mut black_m, mut black_a) = (vec![0; 100005], vec![0; 100005], 0, 0);
        for (black_i, &black_v) in nums.iter().enumerate() {
            let black_v = black_v as usize;
            black_f[black_c[black_v]] -= 1;
            black_c[black_v] += 1;
            black_f[black_c[black_v]] += 1;
            black_m = black_m.max(black_c[black_v]);
            if black_m == 1 || black_m * black_f[black_m] == black_i + 1 - 1 && black_f[1] == 1 || (black_m - 1) * black_f[black_m - 1] == black_i + 1 - black_m && black_f[black_m] == 1 {
                black_a = black_i as i32 + 1;
            }
        }
        black_a
    }
}
```