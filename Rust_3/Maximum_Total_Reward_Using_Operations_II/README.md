# Maximum Total Reward Using Operations II

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation

---

## Problem

<p>You are given an integer array <code>rewardValues</code> of length <code>n</code>, representing the values of rewards.</p>

<p>Initially, your total reward <code>x</code> is 0, and all indices are <strong>unmarked</strong>. You are allowed to perform the following operation <strong>any</strong> number of times:</p>

<ul>
	<li>Choose an <strong>unmarked</strong> index <code>i</code> from the range <code>[0, n - 1]</code>.</li>
	<li>If <code>rewardValues[i]</code> is <strong>greater</strong> than your current total reward <code>x</code>, then add <code>rewardValues[i]</code> to <code>x</code> (i.e., <code>x = x + rewardValues[i]</code>), and <strong>mark</strong> the index <code>i</code>.</li>
</ul>

<p>Return an integer denoting the <strong>maximum </strong><em>total reward</em> you can collect by performing the operations optimally.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">rewardValues = [1,1,3,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>During the operations, we can choose to mark the indices 0 and 2 in order, and the total reward will be 4, which is the maximum.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">rewardValues = [1,6,4,3,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">11</span></p>

<p><strong>Explanation:</strong></p>

<p>Mark the indices 0, 2, and 1 in order. The total reward will then be 11, which is the maximum.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= rewardValues.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= rewardValues[i] &lt;= 5 * 10<sup>4</sup></code></li>
</ul>


## Hints

1. Sort the rewards array first.
2. If we decide to apply some rewards, it's always optimal to apply them in order.
3. The transition is given by: <code>dp[i][j] = dp[i - 1][j − rewardValues[i]]</code> if <code>j − rewardValues[i] < rewardValues[i]</code>.
4. Note that the dp array is a boolean array. We just need 1 bit per element, so we can use a bitset or something similar. We just need a "stream" of bits and apply bitwise operations to optimize the computations by a constant factor.

## Solution

```rust
impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut black_v = reward_values;
        black_v.sort_unstable();
        black_v.dedup();

        let mut black_bs = vec![0u64; 1570];
        black_bs[0] = 1;

        for v in black_v {
            let v = v as usize;
            let black_words = v / 64;
            let black_bits = v % 64;

            if black_bits == 0 {
                for i in 0..black_words {
                    black_bs[i + black_words] |= black_bs[i];
                }
            } else {
                let black_inv_bits = 64 - black_bits;
                for i in 0..black_words {
                    let black_val = black_bs[i];
                    black_bs[i + black_words] |= black_val << black_bits;
                    black_bs[i + black_words + 1] |= black_val >> black_inv_bits;
                }
                let black_mask = (1u64 << black_bits) - 1;
                let black_val = black_bs[black_words] & black_mask;
                black_bs[black_words * 2] |= black_val << black_bits;
                if black_words * 2 + 1 < 1570 {
                    black_bs[black_words * 2 + 1] |= black_val >> black_inv_bits;
                }
            }
        }

        for i in (0..1570).rev() {
            if black_bs[i] != 0 {
                return (i * 64 + 63 - black_bs[i].leading_zeros() as usize) as i32;
            }
        }
        0
    }
}
```