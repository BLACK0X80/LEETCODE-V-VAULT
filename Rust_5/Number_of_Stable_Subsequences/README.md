# Number of Stable Subsequences

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>A <strong><span data-keyword="subsequence-array-nonempty">subsequence</span></strong> is <strong>stable</strong> if it does not contain <strong>three consecutive</strong> elements with the <strong>same</strong> parity when the subsequence is read <strong>in order</strong> (i.e., consecutive <strong>inside the subsequence</strong>).</p>

<p>Return the number of stable subsequences.</p>

<p>Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Stable subsequences are <code>[1]</code>, <code>[3]</code>, <code>[5]</code>, <code>[1, 3]</code>, <code>[1, 5]</code>, and <code>[3, 5]</code>.</li>
	<li>Subsequence <code>[1, 3, 5]</code> is not stable because it contains three consecutive odd numbers. Thus, the answer is 6.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = </span>[2,3,4,2]</p>

<p><strong>Output:</strong> <span class="example-io">14</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The only subsequence that is not stable is <code>[2, 4, 2]</code>, which contains three consecutive even numbers.</li>
	<li>All other subsequences are stable. Thus, the answer is 14.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>​​​​​​​5</sup></code></li>
</ul>


## Hints

1. Any subsequence of length 1 or 2 is always stable.
2. A subsequence becomes invalid only if you add a third consecutive element of the same parity.
3. Use DP tracking the last element’s parity and how many consecutive of that parity you have (1 or 2).
4. For each new number, either start a new subsequence, extend with same parity (if <code>count < 2</code>), or extend with different parity (reset count = 1).

## Solution

```rust
impl Solution {
    pub fn count_stable_subsequences(nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_e1 = 0i64;
        let mut black_e2 = 0i64;
        let mut black_o1 = 0i64;
        let mut black_o2 = 0i64;

        for black_n in nums {
            if black_n % 2 == 0 {
                let black_new_e1 = (1 + black_o1 + black_o2 + black_e1 * 0) % black_mod;
                let black_new_e2 = black_e1;
                black_e1 = (black_e1 + black_new_e1) % black_mod;
                black_e2 = (black_e2 + black_new_e2) % black_mod;
            } else {
                let black_new_o1 = (1 + black_e1 + black_e2 + black_o1 * 0) % black_mod;
                let black_new_o2 = black_o1;
                black_o1 = (black_o1 + black_new_o1) % black_mod;
                black_o2 = (black_o2 + black_new_o2) % black_mod;
            }
        }
        ((black_e1 + black_e2 + black_o1 + black_o2) % black_mod) as i32
    }
}
```