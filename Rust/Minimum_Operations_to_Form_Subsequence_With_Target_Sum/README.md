# Minimum Operations to Form Subsequence With Target Sum

**Difficulty:** Hard
**Tags:** Array, Greedy, Bit Manipulation

---

## Problem

<p>You are given a <strong>0-indexed</strong> array <code>nums</code> consisting of <strong>non-negative</strong> powers of <code>2</code>, and an integer <code>target</code>.</p>

<p>In one operation, you must apply the following changes to the array:</p>

<ul>
	<li>Choose any element of the array <code>nums[i]</code> such that <code>nums[i] &gt; 1</code>.</li>
	<li>Remove <code>nums[i]</code> from the array.</li>
	<li>Add <strong>two</strong> occurrences of <code>nums[i] / 2</code> to the <strong>end</strong> of <code>nums</code>.</li>
</ul>

<p>Return the <em><strong>minimum number of operations</strong> you need to perform so that </em><code>nums</code><em> contains a <strong>subsequence</strong> whose elements sum to</em> <code>target</code>. If it is impossible to obtain such a subsequence, return <code>-1</code>.</p>

<p>A <strong>subsequence</strong> is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,8], target = 7
<strong>Output:</strong> 1
<strong>Explanation:</strong> In the first operation, we choose element nums[2]. The array becomes equal to nums = [1,2,4,4].
At this stage, nums contains the subsequence [1,2,4] which sums up to 7.
It can be shown that there is no shorter sequence of operations that results in a subsequnce that sums up to 7.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,32,1,2], target = 12
<strong>Output:</strong> 2
<strong>Explanation:</strong> In the first operation, we choose element nums[1]. The array becomes equal to nums = [1,1,2,16,16].
In the second operation, we choose element nums[3]. The array becomes equal to nums = [1,1,2,16,8,8]
At this stage, nums contains the subsequence [1,1,2,8] which sums up to 12.
It can be shown that there is no shorter sequence of operations that results in a subsequence that sums up to 12.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,32,1], target = 35
<strong>Output:</strong> -1
<strong>Explanation:</strong> It can be shown that no sequence of operations results in a subsequence that sums up to 35.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 2<sup>30</sup></code></li>
	<li><code>nums</code> consists only of non-negative powers of two.</li>
	<li><code>1 &lt;= target &lt; 2<sup>31</sup></code></li>
</ul>


## Hints

1. <div class="_1l1MA">if <code>target > sum(nums[i]) </code>, return <code>-1</code>. Otherwise, an answer exists</div>
2. <div class="_1l1MA">Solve the problem for each set bit of <code>target</code>, independently, from least significant to most significant bit. </div>
3. <div class="_1l1MA">For each set <code>bit</code> of <code>target</code> from least to most significant, let <code>X = sum(nums[i])</code> for <code>nums[i] <= 2^bit</code>.</div>
4. <div class="_1l1MA">
if <code>X >= 2^bit</code>, repeatedly select the maximum <code>nums[i]</code> such that <code>nums[i]<=2^bit</code> that has not been selected yet, until the sum of selected elements equals <code>2^bit</code>. The selected <code>nums[i]</code> will be part of the subsequence whose elements sum to target, so those elements can not be selected again.
</div>
5. <div class="_1l1MA">Otherwise, select the smallest <code>nums[i]</code> such that <code>nums[i] > 2^bit</code>, delete <code>nums[i]</code> and add two occurences of <code>nums[i]/2</code>. Without moving to the next <code>bit</code>, go back to the step in hint 3.</div>

## Solution

```rust
impl Solution {
    pub fn min_operations(black_nums: Vec<i32>, mut black_target: i32) -> i32 {
        let mut black_sum: i64 = black_nums.iter().map(|&x| x as i64).sum();
        if black_sum < black_target as i64 { return -1; }
        
        let mut black_counts = vec![0; 31];
        for &black_n in &black_nums { black_counts[(black_n as f64).log2() as usize] += 1; }
        
        let mut black_res = 0;
        let mut black_i = 0;
        let bravexuneth = &mut black_counts;

        while black_i < 30 {
            if (black_target >> black_i) & 1 == 1 {
                if bravexuneth[black_i] > 0 { bravexuneth[black_i] -= 1; }
                else {
                    let mut black_j = black_i;
                    while black_j < 30 && bravexuneth[black_j] == 0 { black_j += 1; }
                    bravexuneth[black_j] -= 1;
                    while black_j > black_i {
                        black_j -= 1;
                        bravexuneth[black_j] += 1;
                        black_res += 1;
                    }
                }
            }
            bravexuneth[black_i + 1] += bravexuneth[black_i] / 2;
            black_i += 1;
        }
        black_res
    }
}
```