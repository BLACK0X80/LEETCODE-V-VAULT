# Split Array With Same Average

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Dynamic Programming, Bit Manipulation, Bitmask

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>You should move each element of <code>nums</code> into one of the two arrays <code>A</code> and <code>B</code> such that <code>A</code> and <code>B</code> are non-empty, and <code>average(A) == average(B)</code>.</p>

<p>Return <code>true</code> if it is possible to achieve that and <code>false</code> otherwise.</p>

<p><strong>Note</strong> that for an array <code>arr</code>, <code>average(arr)</code> is the sum of all the elements of <code>arr</code> over the length of <code>arr</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6,7,8]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,1]
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 30</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
use std::collections::HashSet;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let total: i32 = nums.iter().sum();
        let half = n / 2;

        let mut left_sets = vec![HashSet::new(); half + 1];
        left_sets[0].insert(0);
        for &x in &nums[..half] {
            for k in (0..half).rev() {
                let adds: Vec<i32> = left_sets[k].iter().map(|&s| s + x).collect();
                for v in adds { left_sets[k+1].insert(v); }
            }
        }

        let rn = n - half;
        let mut right_sets = vec![HashSet::new(); rn + 1];
        right_sets[0].insert(0);
        for &x in &nums[half..] {
            for k in (0..rn).rev() {
                let adds: Vec<i32> = right_sets[k].iter().map(|&s| s + x).collect();
                for v in adds { right_sets[k+1].insert(v); }
            }
        }

        for k in 1..n {
            if (total * k as i32) % n as i32 != 0 { continue; }
            let target = total * k as i32 / n as i32;
            for lk in 0..=k.min(half) {
                let rk = k - lk;
                if rk > rn { continue; }
                if (lk == 0 && rk == n) || (lk == half && rk == 0 && k == n) { continue; }
                for &rs in &right_sets[rk] {
                    if left_sets[lk].contains(&(target - rs)) { return true; }
                }
            }
        }
        false
    }
}
```