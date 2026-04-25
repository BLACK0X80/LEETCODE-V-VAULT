# The Number of Good Subsets

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Dynamic Programming, Bit Manipulation, Counting, Number Theory, Bitmask

---

## Problem

<p>You are given an integer array <code>nums</code>. We call a subset of <code>nums</code> <strong>good</strong> if its product can be represented as a product of one or more <strong>distinct prime</strong> numbers.</p>

<ul>
	<li>For example, if <code>nums = [1, 2, 3, 4]</code>:

	<ul>
		<li><code>[2, 3]</code>, <code>[1, 2, 3]</code>, and <code>[1, 3]</code> are <strong>good</strong> subsets with products <code>6 = 2*3</code>, <code>6 = 2*3</code>, and <code>3 = 3</code> respectively.</li>
		<li><code>[1, 4]</code> and <code>[4]</code> are not <strong>good</strong> subsets with products <code>4 = 2*2</code> and <code>4 = 2*2</code> respectively.</li>
	</ul>
	</li>
</ul>

<p>Return <em>the number of different <strong>good</strong> subsets in </em><code>nums</code><em> <strong>modulo</strong> </em><code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>subset</strong> of <code>nums</code> is any array that can be obtained by deleting some (possibly none or all) elements from <code>nums</code>. Two subsets are different if and only if the chosen indices to delete are different.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The good subsets are:
- [1,2]: product is 2, which is the product of distinct prime 2.
- [1,2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [1,3]: product is 3, which is the product of distinct prime 3.
- [2]: product is 2, which is the product of distinct prime 2.
- [2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [3]: product is 3, which is the product of distinct prime 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,2,3,15]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The good subsets are:
- [2]: product is 2, which is the product of distinct prime 2.
- [2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [2,15]: product is 30, which is the product of distinct primes 2, 3, and 5.
- [3]: product is 3, which is the product of distinct prime 3.
- [15]: product is 15, which is the product of distinct primes 3 and 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 30</code></li>
</ul>


## Hints

1. Consider only the numbers which have a good prime factorization.
2. Use brute force to find all possible good subsets and then calculate its frequency in nums.

## Solution

```rust
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let black_primes = [2,3,5,7,11,13,17,19,23,29];
        let mut black_freq = [0i64; 31];
        for &x in &nums { black_freq[x as usize] += 1; }

        let black_mask = |x: i32| -> i32 {
            let mut m = 0;
            for (i, &p) in black_primes.iter().enumerate() {
                if x % p == 0 {
                    if (x / p) % p == 0 { return -1; }
                    m |= 1 << i;
                }
            }
            m
        };

        let mut black_dp = vec![0i64; 1 << 10];
        black_dp[0] = 1;

        for x in 2..=30i32 {
            if black_freq[x as usize] == 0 { continue; }
            let m = black_mask(x);
            if m < 0 { continue; }
            let m = m as usize;
            for s in (0..(1<<10)).rev() {
                if black_dp[s] == 0 { continue; }
                if s & m == 0 {
                    black_dp[s | m] = (black_dp[s | m] + black_dp[s] * black_freq[x as usize]) % MOD;
                }
            }
        }

        let mut black_ans: i64 = black_dp[1..].iter().sum::<i64>() % MOD;
        let mut black_pow = 1i64;
        for _ in 0..black_freq[1] { black_pow = black_pow * 2 % MOD; }
        black_ans = black_ans * black_pow % MOD;
        black_ans as i32
    }
}
```