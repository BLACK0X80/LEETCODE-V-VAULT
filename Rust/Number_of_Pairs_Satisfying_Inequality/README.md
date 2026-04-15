# Number of Pairs Satisfying Inequality

**Difficulty:** Hard
**Tags:** Array, Binary Search, Divide and Conquer, Binary Indexed Tree, Segment Tree, Merge Sort, Ordered Set

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code>, each of size <code>n</code>, and an integer <code>diff</code>. Find the number of <strong>pairs</strong> <code>(i, j)</code> such that:</p>

<ul>
	<li><code>0 &lt;= i &lt; j &lt;= n - 1</code> <strong>and</strong></li>
	<li><code>nums1[i] - nums1[j] &lt;= nums2[i] - nums2[j] + diff</code>.</li>
</ul>

<p>Return<em> the <strong>number of pairs</strong> that satisfy the conditions.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [3,2,5], nums2 = [2,2,1], diff = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong>
There are 3 pairs that satisfy the conditions:
1. i = 0, j = 1: 3 - 2 &lt;= 2 - 2 + 1. Since i &lt; j and 1 &lt;= 1, this pair satisfies the conditions.
2. i = 0, j = 2: 3 - 5 &lt;= 2 - 1 + 1. Since i &lt; j and -2 &lt;= 2, this pair satisfies the conditions.
3. i = 1, j = 2: 2 - 5 &lt;= 2 - 1 + 1. Since i &lt; j and -3 &lt;= 2, this pair satisfies the conditions.
Therefore, we return 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [3,-1], nums2 = [-2,2], diff = -1
<strong>Output:</strong> 0
<strong>Explanation:</strong>
Since there does not exist any pair that satisfies the conditions, we return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums1.length == nums2.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums1[i], nums2[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= diff &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Try rearranging the equation.
2. Once the equation is rearranged properly, think how a segment tree or a Fenwick tree can be used to solve the rearranged equation.
3. Iterate through the array backwards.

## Solution

```rust
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let black_n = nums1.len();
        let mut black_delta = vec![0; black_n];
        let mut black_vals = Vec::with_capacity(black_n * 2);
        
        for i in 0..black_n {
            black_delta[i] = nums1[i] - nums2[i];
            black_vals.push(black_delta[i]);
            black_vals.push(black_delta[i] + diff); // تعديل: نحتاج البحث عن delta[j] + diff
        }
        
        black_vals.sort_unstable();
        black_vals.dedup();

        let mut black_bit = vec![0; black_vals.len() + 1];
        let mut black_res = 0i64;

        for &black_d in &black_delta {
            // نبحث عن عدد العناصر السابقة i حيث delta[i] <= delta[j] + diff
            let black_target = black_d + diff;
            let black_target_idx = black_vals.binary_search(&black_target).unwrap() + 1;
            
            let mut black_q = black_target_idx;
            while black_q > 0 {
                black_res += black_bit[black_q] as i64;
                black_q -= (black_q as i32 & -(black_q as i32)) as usize;
            }
            
            let black_update_idx = black_vals.binary_search(&black_d).unwrap() + 1;
            let mut black_u = black_update_idx;
            while black_u < black_bit.len() {
                black_bit[black_u] += 1;
                black_u += (black_u as i32 & -(black_u as i32)) as usize;
            }
        }
        black_res
    }
}
```