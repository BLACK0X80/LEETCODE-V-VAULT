# Power of Heroes

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Sorting, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> representing the strength of some heroes. The<b> power</b> of a group of heroes is defined as follows:</p>

<ul>
	<li>Let <code>i<sub>0</sub></code>, <code>i<sub>1</sub></code>, ... ,<code>i<sub>k</sub></code> be the indices of the heroes in a group. Then, the power of this group is <code>max(nums[i<sub>0</sub>], nums[i<sub>1</sub>], ... ,nums[i<sub>k</sub>])<sup>2</sup> * min(nums[i<sub>0</sub>], nums[i<sub>1</sub>], ... ,nums[i<sub>k</sub>])</code>.</li>
</ul>

<p>Return <em>the sum of the <strong>power</strong> of all <strong>non-empty</strong> groups of heroes possible.</em> Since the sum could be very large, return it <strong>modulo</strong> <code>10<sup>9 </sup>+ 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,1,4]
<strong>Output:</strong> 141
<strong>Explanation:</strong> 
1<sup>st</sup>&nbsp;group: [2] has power = 2<sup>2</sup>&nbsp;* 2 = 8.
2<sup>nd</sup>&nbsp;group: [1] has power = 1<sup>2</sup> * 1 = 1. 
3<sup>rd</sup>&nbsp;group: [4] has power = 4<sup>2</sup> * 4 = 64. 
4<sup>th</sup>&nbsp;group: [2,1] has power = 2<sup>2</sup> * 1 = 4. 
5<sup>th</sup>&nbsp;group: [2,4] has power = 4<sup>2</sup> * 2 = 32. 
6<sup>th</sup>&nbsp;group: [1,4] has power = 4<sup>2</sup> * 1 = 16. 
​​​​​​​7<sup>th</sup>&nbsp;group: [2,1,4] has power = 4<sup>2</sup>​​​​​​​ * 1 = 16. 
The sum of powers of all groups is 8 + 1 + 64 + 4 + 32 + 16 + 16 = 141.

</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 7
<strong>Explanation:</strong> A total of 7 groups are possible, and the power of each group will be 1. Therefore, the sum of the powers of all groups is 7.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Try something with sorting the array.
2. For a pair of array elements nums[i] and nums[j] (i < j), the power would be nums[i]*nums[j]^2 regardless of how many elements in between are included.
3. The number of subsets with the above as power will correspond to 2^(j-i-1).
4. Try collecting the terms for nums[0], nums[1], …, nums[j-1] when computing the power of heroes ending at index j to get the power in a single pass.

## Solution

```rust
impl Solution {
    pub fn sum_of_power(mut black_nums: Vec<i32>) -> i32 {
        black_nums.sort_unstable();
        let black_mod = 1_000_000_007i64;
        let mut black_res = 0i64;
        let mut black_sum = 0i64;
        let bravexuneth = &black_nums;

        for &black_x in bravexuneth {
            let black_val = black_x as i64;
            let black_sq = (black_val * black_val) % black_mod;
            black_res = (black_res + black_sq % black_mod * (black_sum + black_val) % black_mod) % black_mod;
            black_sum = (black_sum * 2 + black_val) % black_mod;
        }
        black_res as i32
    }
}
```