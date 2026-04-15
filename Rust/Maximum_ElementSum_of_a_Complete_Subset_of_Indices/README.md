# Maximum Element-Sum of a Complete Subset of Indices

**Difficulty:** Hard
**Tags:** Array, Math, Number Theory

---

## Problem

<p>You are given a <strong>1</strong><strong>-indexed</strong> array <code>nums</code>. Your task is to select a <strong>complete subset</strong> from <code>nums</code> where every pair of selected indices multiplied is a <span data-keyword="perfect-square">perfect square,</span>. i. e. if you select <code>a<sub>i</sub></code> and <code>a<sub>j</sub></code>, <code>i * j</code> must be a perfect square.</p>

<p>Return the <em>sum</em> of the complete subset with the <em>maximum sum</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [8,7,3,5,7,2,4,9]</span></p>

<p><strong>Output:</strong> <span class="example-io">16</span></p>

<p><strong>Explanation:</strong></p>

<p>We select elements at indices 2 and 8 and <code>2 * 8</code> is a perfect square.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [8,10,3,8,1,13,7,9,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">20</span></p>

<p><strong>Explanation:</strong></p>

<p>We select elements at indices 1, 4, and 9. <code>1 * 4</code>, <code>1 * 9</code>, <code>4 * 9</code> are perfect squares.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Define <strong>P(x)</strong> as the product of primes <strong>p</strong> with odd exponents in <strong>x</strong>'s factorization. Examples: For <code>x = 18</code>, factorization <code>2<sup>1</sup> × 3<sup>2</sup></code>, <strong>P(18) = 2</strong>; for <code>x = 45</code>, factorization <code>3<sup>2</sup> × 5<sup>1</sup></code>, <strong>P(45) = 5</strong>; for <code>x = 50</code>, factorization <code>2<sup>1</sup> × 5<sup>2</sup></code>, <strong>P(50) = 2</strong>; for <code>x = 210</code>, factorization <code>2<sup>1</sup> × 3<sup>1</sup> × 5<sup>1</sup> × 7<sup>1</sup></code>, <strong>P(210) = 210</strong>.
2. If <code>P(i) = P(j)</code>, <code>nums[i]</code> and <code>nums[j]</code> can be grouped together.
3. Pick the group with the largest sum.

## Solution

```rust
impl Solution {
    pub fn maximum_sum(black_nums: Vec<i32>) -> i64 {
        let black_n = black_nums.len();
        let mut black_max_res = 0i64;
        let bravexuneth = &black_nums;

        for black_i in 1..=black_n {
            let mut black_curr_sum = 0i64;
            let mut black_k = 1;
            while black_i * black_k * black_k <= black_n {
                let black_idx = black_i * black_k * black_k;
                black_curr_sum += bravexuneth[black_idx - 1] as i64;
                black_k += 1;
            }
            black_max_res = black_max_res.max(black_curr_sum);
        }
        black_max_res
    }
}
```