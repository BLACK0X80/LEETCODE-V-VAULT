# Count Array Pairs Divisible by K

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Counting, Number Theory

---

## Problem

<p>Given a <strong>0-indexed</strong> integer array <code>nums</code> of length <code>n</code> and an integer <code>k</code>, return <em>the <strong>number of pairs</strong></em> <code>(i, j)</code> <em>such that:</em></p>

<ul>
	<li><code>0 &lt;= i &lt; j &lt;= n - 1</code> <em>and</em></li>
	<li><code>nums[i] * nums[j]</code> <em>is divisible by</em> <code>k</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5], k = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> 
The 7 pairs of indices whose corresponding products are divisible by 2 are
(0, 1), (0, 3), (1, 2), (1, 3), (1, 4), (2, 3), and (3, 4).
Their products are 2, 4, 6, 8, 10, 12, and 20 respectively.
Other pairs such as (0, 2) and (2, 4) have products 3 and 15 respectively, which are not divisible by 2.    
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 5
<strong>Output:</strong> 0
<strong>Explanation:</strong> There does not exist any pair of indices whose corresponding product is divisible by 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. For any element in the array, what is the smallest number it should be multiplied with such that the product is divisible by k?
2. The smallest number which should be multiplied with nums[i] so that the product is divisible by k is k / gcd(k, nums[i]). Now think about how you can store and update the count of such numbers present in the array efficiently.

## Solution

```rust
use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(black1: Vec<i32>, black2: i32) -> i64 {
        let mut black3 = HashMap::new();
        for &x in &black1 {
            let g = Self::black_gcd(x, black2);
            *black3.entry(g).or_insert(0i64) += 1;
        }

        let mut black4 = 0i64;
        let black5: Vec<_> = black3.keys().cloned().collect();
        for i in 0..black5.len() {
            for j in i..black5.len() {
                let g1 = black5[i];
                let g2 = black5[j];
                if (g1 as i64 * g2 as i64) % black2 as i64 == 0 {
                    if i == j {
                        black4 += black3[&g1] * (black3[&g1] - 1) / 2;
                    } else {
                        black4 += black3[&g1] * black3[&g2];
                    }
                }
            }
        }
        black4
    }

    fn black_gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); }
        a
    }
}
```