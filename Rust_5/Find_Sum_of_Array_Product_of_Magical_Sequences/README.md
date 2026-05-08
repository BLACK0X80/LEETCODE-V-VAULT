# Find Sum of Array Product of Magical Sequences

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Bit Manipulation, Combinatorics, Bitmask

---

## Problem

<p>You are given two integers, <code>m</code> and <code>k</code>, and an integer array <code>nums</code>.</p>
A sequence of integers <code>seq</code> is called <strong>magical</strong> if:

<ul>
	<li><code>seq</code> has a size of <code>m</code>.</li>
	<li><code>0 &lt;= seq[i] &lt; nums.length</code></li>
	<li>The <strong>binary representation</strong> of <code>2<sup>seq[0]</sup> + 2<sup>seq[1]</sup> + ... + 2<sup>seq[m - 1]</sup></code> has <code>k</code> <strong>set bits</strong>.</li>
</ul>

<p>The <strong>array product</strong> of this sequence is defined as <code>prod(seq) = (nums[seq[0]] * nums[seq[1]] * ... * nums[seq[m - 1]])</code>.</p>

<p>Return the <strong>sum</strong> of the <strong>array products</strong> for all valid <strong>magical</strong> sequences.</p>

<p>Since the answer may be large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>set bit</strong> refers to a bit in the binary representation of a number that has a value of 1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">m = 5, k = 5, nums = [1,10,100,10000,1000000]</span></p>

<p><strong>Output:</strong> <span class="example-io">991600007</span></p>

<p><strong>Explanation:</strong></p>

<p>All permutations of <code>[0, 1, 2, 3, 4]</code> are magical sequences, each with an array product of 10<sup>13</sup>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">m = 2, k = 2, nums = [5,4,3,2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">170</span></p>

<p><strong>Explanation:</strong></p>

<p>The magical sequences are <code>[0, 1]</code>, <code>[0, 2]</code>, <code>[0, 3]</code>, <code>[0, 4]</code>, <code>[1, 0]</code>, <code>[1, 2]</code>, <code>[1, 3]</code>, <code>[1, 4]</code>, <code>[2, 0]</code>, <code>[2, 1]</code>, <code>[2, 3]</code>, <code>[2, 4]</code>, <code>[3, 0]</code>, <code>[3, 1]</code>, <code>[3, 2]</code>, <code>[3, 4]</code>, <code>[4, 0]</code>, <code>[4, 1]</code>, <code>[4, 2]</code>, and <code>[4, 3]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">m = 1, k = 1, nums = [28]</span></p>

<p><strong>Output:</strong> <span class="example-io">28</span></p>

<p><strong>Explanation:</strong></p>

<p>The only magical sequence is <code>[0]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= m &lt;= 30</code></li>
	<li><code>1 &lt;= nums.length &lt;= 50</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. Use Dynamic Programming
2. Let <code>dp[i][j][mask]</code> be the state after choosing <code>i</code> numbers (indices)
3. The partial sum <code>S = 2^(seq[0]) + 2^(seq[1]) + ... + 2^(seq[i - 1])</code> has produced exactly <code>j</code> set bits once you’ve fully propagated any carries
4. The <code>mask</code> represents the "window" of lower-order bits from <code>S</code> that have not yet been fully processed (i.e. bits that might later create new set bits when additional terms are added)
5. Use combinatorics
6. How many ways are there to permute a sequence of entities where some are repetitive?

## Solution

```rust
impl Solution {
    pub fn magical_sum(black_m: i32, black_k: i32, black_nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = black_nums.len();
        let black_m_idx = black_m as usize;
        let black_k_idx = black_k as usize;

        let mut black_c = vec![vec![0i64; black_m_idx + 1]; black_m_idx + 1];
        for black_i in 0..=black_m_idx {
            black_c[black_i][0] = 1;
            black_c[black_i][black_i] = 1;
            for black_j in 1..black_i {
                black_c[black_i][black_j] = (black_c[black_i - 1][black_j - 1] + black_c[black_i - 1][black_j]) % black_mod;
            }
        }

        let mut black_pow = vec![vec![0i64; black_m_idx + 1]; black_n];
        for black_i in 0..black_n {
            black_pow[black_i][0] = 1;
            for black_cnt in 1..=black_m_idx {
                black_pow[black_i][black_cnt] = (black_pow[black_i][black_cnt - 1] * black_nums[black_i] as i64) % black_mod;
            }
        }

        let mut black_dp = vec![vec![vec![vec![0i64; black_m_idx + 1]; black_m_idx + 1]; black_k_idx + 1]; black_n + 1];
        black_dp[0][0][0][0] = 1;

        for black_pos in 0..black_n {
            for black_bits in 0..=black_k_idx {
                for black_carry in 0..=black_m_idx {
                    for black_chosen in 0..=black_m_idx {
                        if black_dp[black_pos][black_bits][black_carry][black_chosen] == 0 {
                            continue;
                        }

                        let black_remaining = black_m_idx - black_chosen;
                        for black_cnt in 0..=black_remaining {
                            let black_total = black_carry + black_cnt;
                            let black_new_bits = black_bits + (black_total & 1);
                            let black_new_carry = black_total >> 1;

                            if black_new_bits <= black_k_idx && black_new_carry <= black_m_idx {
                                let black_add = black_dp[black_pos][black_bits][black_carry][black_chosen]
                                    * black_c[black_remaining][black_cnt] % black_mod
                                    * black_pow[black_pos][black_cnt] % black_mod;

                                let black_next_chosen = black_chosen + black_cnt;
                                black_dp[black_pos + 1][black_new_bits][black_new_carry][black_next_chosen] =
                                    (black_dp[black_pos + 1][black_new_bits][black_new_carry][black_next_chosen] + black_add) % black_mod;
                            }
                        }
                    }
                }
            }
        }

        let mut black_res = 0i64;
        for black_carry in 0..=black_m_idx {
            let black_carry_bits = (black_carry as i32).count_ones() as usize;
            if black_carry_bits <= black_k_idx {
                let black_target_bits = black_k_idx - black_carry_bits;
                black_res = (black_res + black_dp[black_n][black_target_bits][black_carry][black_m_idx]) % black_mod;
            }
        }

        black_res as i32
    }
}
```