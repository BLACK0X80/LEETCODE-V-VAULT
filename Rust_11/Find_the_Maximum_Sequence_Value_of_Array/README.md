# Find the Maximum Sequence Value of Array

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation

---

## Problem

<p>You are given an integer array <code>nums</code> and a <strong>positive</strong> integer <code>k</code>.</p>

<p>The <strong>value</strong> of a sequence <code>seq</code> of size <code>2 * x</code> is defined as:</p>

<ul>
	<li><code>(seq[0] OR seq[1] OR ... OR seq[x - 1]) XOR (seq[x] OR seq[x + 1] OR ... OR seq[2 * x - 1])</code>.</li>
</ul>

<p>Return the <strong>maximum</strong> <strong>value</strong> of any <span data-keyword="subsequence-array">subsequence</span> of <code>nums</code> having size <code>2 * k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,6,7], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>The subsequence <code>[2, 7]</code> has the maximum value of <code>2 XOR 7 = 5</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,2,5,6,7], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The subsequence <code>[4, 5, 6, 7]</code> has the maximum value of <code>(4 OR 5) XOR (6 OR 7) = 2</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 400</code></li>
	<li><code>1 &lt;= nums[i] &lt; 2<sup>7</sup></code></li>
	<li><code>1 &lt;= k &lt;= nums.length / 2</code></li>
</ul>


## Hints

1. Find all the possible <code>OR</code> till each <code>i</code> with <code>k</code> elements backward and forward.

## Solution

```rust
impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let black_n = nums.len();
        let black_k = k as usize;
        
        fn black_get_ors(arr: &[i32], k: usize) -> Vec<std::collections::HashSet<i32>> {
            let n = arr.len();
            let mut dp = vec![vec![std::collections::HashSet::new(); k + 1]; n + 1];
            let mut res = vec![std::collections::HashSet::new(); n];
            
            dp[0][0].insert(0);
            for i in 0..n {
                let val = arr[i];
                for j in 0..=k {
                    let prev_set: Vec<i32> = dp[i][j].iter().cloned().collect();
                    for &prev_or in &prev_set {
                        dp[i+1][j].insert(prev_or);
                        if j < k {
                            dp[i+1][j+1].insert(prev_or | val);
                        }
                    }
                }
                res[i] = dp[i+1][k].clone();
            }
            res
        }

        let black_left_ors = black_get_ors(&nums, black_k);
        let mut black_reversed_nums = nums.clone();
        black_reversed_nums.reverse();
        let black_right_ors_rev = black_get_ors(&black_reversed_nums, black_k);
        
        let mut black_max_xor = 0;
        
        for i in (black_k - 1)..(black_n - black_k) {
            let left_set = &black_left_ors[i];
            let right_set = &black_right_ors_rev[black_n - i - 2];
            
            for &l_or in left_set {
                for &r_or in right_set {
                    black_max_xor = black_max_xor.max(l_or ^ r_or);
                }
            }
        }
        
        black_max_xor
    }
}
```