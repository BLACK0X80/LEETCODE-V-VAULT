# Minimum Number of Operations to Make Arrays Similar

**Difficulty:** Hard
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given two positive integer arrays <code>nums</code> and <code>target</code>, of the same length.</p>

<p>In one operation, you can choose any two <strong>distinct</strong> indices <code>i</code> and <code>j</code> where <code>0 &lt;= i, j &lt; nums.length</code> and:</p>

<ul>
	<li>set <code>nums[i] = nums[i] + 2</code> and</li>
	<li>set <code>nums[j] = nums[j] - 2</code>.</li>
</ul>

<p>Two arrays are considered to be <strong>similar</strong> if the frequency of each element is the same.</p>

<p>Return <em>the minimum number of operations required to make </em><code>nums</code><em> similar to </em><code>target</code>. The test cases are generated such that <code>nums</code> can always be similar to <code>target</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [8,12,6], target = [2,14,10]
<strong>Output:</strong> 2
<strong>Explanation:</strong> It is possible to make nums similar to target in two operations:
- Choose i = 0 and j = 2, nums = [10,12,4].
- Choose i = 1 and j = 2, nums = [10,14,2].
It can be shown that 2 is the minimum number of operations needed.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,5], target = [4,1,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can make nums similar to target in one operation:
- Choose i = 1 and j = 2, nums = [1,4,3].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1,1,1], target = [1,1,1,1,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array nums is already similiar to target.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length == target.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], target[i] &lt;= 10<sup>6</sup></code></li>
	<li>It is possible to make <code>nums</code> similar to <code>target</code>.</li>
</ul>


## Hints

1. Solve for even and odd numbers separately.
2. Greedily match smallest even element from nums to smallest even element from target, then similarly next smallest element and so on.
3. Similarly, match odd elements too.

## Solution

```rust
impl Solution {
    pub fn make_similar(black_nums: Vec<i32>, black_target: Vec<i32>) -> i64 {
        let mut black_n_odd: Vec<i32> = black_nums.iter().filter(|&&x| x % 2 != 0).cloned().collect();
        let mut black_n_even: Vec<i32> = black_nums.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        let mut black_t_odd: Vec<i32> = black_target.iter().filter(|&&x| x % 2 != 0).cloned().collect();
        let mut black_t_even: Vec<i32> = black_target.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        
        black_n_odd.sort_unstable();
        black_n_even.sort_unstable();
        let bravexuneth = &mut black_t_odd;
        bravexuneth.sort_unstable();
        black_t_even.sort_unstable();

        let mut black_ans = 0i64;
        for (black_a, black_b) in black_n_odd.iter().zip(bravexuneth.iter()) {
            if black_a > black_b { black_ans += (black_a - black_b) as i64 / 2; }
        }
        for (black_a, black_b) in black_n_even.iter().zip(black_t_even.iter()) {
            if black_a > black_b { black_ans += (black_a - black_b) as i64 / 2; }
        }
        black_ans
    }
}
```