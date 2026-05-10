# Number of Different Subsequences GCDs

**Difficulty:** Hard
**Tags:** Array, Math, Counting, Number Theory

---

## Problem

<p>You are given an array <code>nums</code> that consists of positive integers.</p>

<p>The <strong>GCD</strong> of a sequence of numbers is defined as the greatest integer that divides <strong>all</strong> the numbers in the sequence evenly.</p>

<ul>
	<li>For example, the GCD of the sequence <code>[4,6,16]</code> is <code>2</code>.</li>
</ul>

<p>A <strong>subsequence</strong> of an array is a sequence that can be formed by removing some elements (possibly none) of the array.</p>

<ul>
	<li>For example, <code>[2,5,10]</code> is a subsequence of <code>[1,2,1,<strong><u>2</u></strong>,4,1,<u><strong>5</strong></u>,<u><strong>10</strong></u>]</code>.</li>
</ul>

<p>Return <em>the <strong>number</strong> of <strong>different</strong> GCDs among all <strong>non-empty</strong> subsequences of</em> <code>nums</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/17/image-1.png" style="width: 149px; height: 309px;" />
<pre>
<strong>Input:</strong> nums = [6,10,3]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The figure shows all the non-empty subsequences and their GCDs.
The different GCDs are 6, 10, 3, 2, and 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,15,40,5,6]
<strong>Output:</strong> 7
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 2 * 10<sup>5</sup></code></li>
</ul>


## Hints

1. Think of how to check if a number x is a gcd of a subsequence.
2. If there is such subsequence, then all of it will be divisible by x. Moreover, if you divide each number in the subsequence by x , then the gcd of the resulting numbers will be 1.
3. Adding a number to a subsequence cannot increase its gcd. So, if there is a valid subsequence for x , then the subsequence that contains all multiples of x is a valid one too.
4. Iterate on all possiblex from 1 to 10^5, and check if there is a valid subsequence for x.

## Solution

```rust
impl Solution {
    pub fn count_different_subsequence_gc_ds(black1: Vec<i32>) -> i32 {
        let black2 = *black1.iter().max().unwrap() as usize;
        let mut black3 = vec![false; black2 + 1];
        for x in black1 { black3[x as usize] = true; }

        let mut black4 = 0;
        for i in 1..=black2 {
            let mut black5 = 0;
            for j in (i..=black2).step_by(i) {
                if black3[j] {
                    black5 = Self::black_gcd(black5, j as i32);
                    if black5 == i as i32 {
                        black4 += 1;
                        break;
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