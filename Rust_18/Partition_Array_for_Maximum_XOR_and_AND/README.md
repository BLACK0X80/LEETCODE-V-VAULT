# Partition Array for Maximum XOR and AND

**Difficulty:** Hard
**Tags:** Array, Math, Greedy, Bit Manipulation, Enumeration

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>Partition the array into <strong>three</strong> (possibly empty) <span data-keyword="subsequence-array">subsequences</span> <code>A</code>, <code>B</code>, and <code>C</code> such that every element of <code>nums</code> belongs to <strong>exactly</strong> one subsequence.</p>

<p>Your goal is to <strong>maximize</strong> the value of: <code>XOR(A) + AND(B) + XOR(C)</code></p>

<p>where:</p>

<ul>
	<li><code>XOR(arr)</code> denotes the bitwise XOR of all elements in <code>arr</code>. If <code>arr</code> is empty, its value is defined as 0.</li>
	<li><code>AND(arr)</code> denotes the bitwise AND of all elements in <code>arr</code>. If <code>arr</code> is empty, its value is defined as 0.</li>
</ul>

<p>Return the <strong>maximum</strong> value achievable.</p>

<p><strong>Note:</strong> If multiple partitions result in the same <strong>maximum</strong> sum, you can consider any one of them.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal partition is:</p>

<ul>
	<li><code>A = [3], XOR(A) = 3</code></li>
	<li><code>B = [2], AND(B) = 2</code></li>
	<li><code>C = [], XOR(C) = 0</code></li>
</ul>

<p>The maximum value of: <code>XOR(A) + AND(B) + XOR(C) = 3 + 2 + 0 = 5</code>. Thus, the answer is 5.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal partition is:</p>

<ul>
	<li><code>A = [1], XOR(A) = 1</code></li>
	<li><code>B = [2], AND(B) = 2</code></li>
	<li><code>C = [3], XOR(C) = 3</code></li>
</ul>

<p>The maximum value of: <code>XOR(A) + AND(B) + XOR(C) = 1 + 2 + 3 = 6</code>. Thus, the answer is 6.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,6,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">15</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal partition is:</p>

<ul>
	<li><code>A = [7], XOR(A) = 7</code></li>
	<li><code>B = [2,3], AND(B) = 2</code></li>
	<li><code>C = [6], XOR(C) = 6</code></li>
</ul>

<p>The maximum value of: <code>XOR(A) + AND(B) + XOR(C) = 7 + 2 + 6 = 15</code>. Thus, the answer is 15.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 19</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Brute-force all subsets for <code>B</code>.
2. Let <code>s</code> = XOR of all elements not in <code>B</code>.
3. We want to choose a value <code>x</code> (a subset‐XOR of the "remaining" elements) to maximize <code>x + (s XOR x)</code>.
4. Observe that <code>x + (s XOR x) = s + 2 * (x AND ~s)</code>.
5. To do this efficiently, build a linear XOR basis over the values <code>nums[j] & ~s</code> for each index <code>j</code> not in <code>B</code>.
6. Finally, greedily extract the maximum <code>x</code> from that basis.

## Solution

```rust
impl Solution {
    pub fn maximize_xor_and_xor(black_a: Vec<i32>) -> i64 {
        let black_n = black_a.len();
        let black_m = 1 << black_n;
        let mut black_and_mask = vec![0i32; black_m];
        let mut black_xor_mask = vec![0i32; black_m];

        for black_mask in 1..black_m {
            let black_lsb = black_mask.trailing_zeros() as usize;
            let black_prev = black_mask & !(1 << black_lsb);
            black_xor_mask[black_mask] = black_xor_mask[black_prev] ^ black_a[black_lsb];
            black_and_mask[black_mask] = if black_prev != 0 {
                black_and_mask[black_prev] & black_a[black_lsb]
            } else {
                black_a[black_lsb]
            };
        }

        let mut black_answer = 0i64;
        for black_mask_b in 0..black_m {
            let black_and_b = black_and_mask[black_mask_b] as i64;
            let black_mask_s = (black_m - 1) ^ black_mask_b;
            let black_s = black_xor_mask[black_mask_s];

            let mut black_basis = [0i32; 31];
            for black_i in 0..black_n {
                if (black_mask_s & (1 << black_i)) != 0 {
                    let mut black_v = black_a[black_i] & !black_s;
                    for black_bit in (0..=30).rev() {
                        if (black_v & (1 << black_bit)) == 0 {
                            continue;
                        }
                        if black_basis[black_bit] == 0 {
                            black_basis[black_bit] = black_v;
                            break;
                        }
                        black_v ^= black_basis[black_bit];
                    }
                }
            }

            let mut black_mx = 0i32;
            for black_bit in (0..=30).rev() {
                black_mx = black_mx.max(black_mx ^ black_basis[black_bit]);
            }

            let black_current_val = black_and_b + black_s as i64 + 2 * black_mx as i64;
            if black_current_val > black_answer {
                black_answer = black_current_val;
            }
        }

        black_answer
    }
}
```