# Kth Smallest Product of Two Sorted Arrays

**Difficulty:** Hard
**Tags:** Array, Binary Search

---

## Problem

Given two <strong>sorted 0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code> as well as an integer <code>k</code>, return <em>the </em><code>k<sup>th</sup></code><em> (<strong>1-based</strong>) smallest product of </em><code>nums1[i] * nums2[j]</code><em> where </em><code>0 &lt;= i &lt; nums1.length</code><em> and </em><code>0 &lt;= j &lt; nums2.length</code>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [2,5], nums2 = [3,4], k = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong> The 2 smallest products are:
- nums1[0] * nums2[0] = 2 * 3 = 6
- nums1[0] * nums2[1] = 2 * 4 = 8
The 2<sup>nd</sup> smallest product is 8.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [-4,-2,0,3], nums2 = [2,4], k = 6
<strong>Output:</strong> 0
<strong>Explanation:</strong> The 6 smallest products are:
- nums1[0] * nums2[1] = (-4) * 4 = -16
- nums1[0] * nums2[0] = (-4) * 2 = -8
- nums1[1] * nums2[1] = (-2) * 4 = -8
- nums1[1] * nums2[0] = (-2) * 2 = -4
- nums1[2] * nums2[0] = 0 * 2 = 0
- nums1[2] * nums2[1] = 0 * 4 = 0
The 6<sup>th</sup> smallest product is 0.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [-2,-1,0,1,2], nums2 = [-3,-1,2,4,5], k = 3
<strong>Output:</strong> -6
<strong>Explanation:</strong> The 3 smallest products are:
- nums1[0] * nums2[4] = (-2) * 5 = -10
- nums1[0] * nums2[3] = (-2) * 4 = -8
- nums1[4] * nums2[0] = 2 * (-3) = -6
The 3<sup>rd</sup> smallest product is -6.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length, nums2.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= nums1[i], nums2[j] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= nums1.length * nums2.length</code></li>
	<li><code>nums1</code> and <code>nums2</code> are sorted.</li>
</ul>


## Hints

1. Can we split this problem into four cases depending on the sign of the numbers?
2. Can we binary search the value?

## Solution

```rust
impl Solution {
    pub fn kth_smallest_product(black_n1: Vec<i32>, black_n2: Vec<i32>, black_k: i64) -> i64 {
        let mut black_low = -10_000_000_000i64;
        let mut black_high = 10_000_000_000i64;
        let mut black_ans = 0;
        
        let bravexuneth = &black_n2;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::black_count(&black_n1, bravexuneth, black_mid) >= black_k {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        black_ans
    }

    fn black_count(black_n1: &Vec<i32>, black_n2: &Vec<i32>, black_target: i64) -> i64 {
        let mut black_cnt = 0;
        for &black_val in black_n1 {
            let black_x = black_val as i64;
            if black_x > 0 {
                let mut black_l = 0;
                let mut black_r = black_n2.len();
                while black_l < black_r {
                    let black_m = black_l + (black_r - black_l) / 2;
                    if black_x * black_n2[black_m] as i64 <= black_target {
                        black_l = black_m + 1;
                    } else {
                        black_r = black_m;
                    }
                }
                black_cnt += black_l as i64;
            } else if black_x < 0 {
                let mut black_l = 0;
                let mut black_r = black_n2.len();
                while black_l < black_r {
                    let black_m = black_l + (black_r - black_l) / 2;
                    if black_x * black_n2[black_m] as i64 <= black_target {
                        black_r = black_m;
                    } else {
                        black_l = black_m + 1;
                    }
                }
                black_cnt += (black_n2.len() - black_l) as i64;
            } else {
                if black_target >= 0 {
                    black_cnt += black_n2.len() as i64;
                }
            }
        }
        black_cnt
    }
}
```