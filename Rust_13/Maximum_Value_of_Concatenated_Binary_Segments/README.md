# Maximum Value of Concatenated Binary Segments

**Difficulty:** Hard
**Tags:** 

---

## Problem

<p>You are given two integer arrays <code>nums1</code> and <code>nums0</code>, each of size <code>n</code>.</p>

<ul>
	<li><code>nums1[i]</code> represents the number of <code>&#39;1&#39;</code>s in the <code>i<sup>th</sup></code> segment.</li>
	<li><code>nums0[i]</code> represents the number of <code>&#39;0&#39;</code>s in the <code>i<sup>th</sup></code> segment.</li>
</ul>

<p>For each index <code>i</code>, construct a binary segment consisting of:</p>

<ul>
	<li><code>nums1[i]</code> occurrences of <code>&#39;1&#39;</code> followed by</li>
	<li><code>nums0[i]</code> occurrences of <code>&#39;0&#39;</code>.</li>
</ul>

<p>You may <strong>rearrange</strong> the order of these <strong>segments</strong> in any way. After rearranging, <strong>concatenate</strong> all segments to form a single binary string.</p>

<p>Return the <strong>maximum</strong> possible integer value of the concatenated binary string.</p>

<p>Since the result can be very large, return the answer <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums1 = [1,2], nums0 = [1,0]</span></p>

<p><strong>Output:</strong> <span class="example-io">14</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>At index 0, <code>nums1[0] = 1</code> and <code>nums0[0] = 1</code>, so the segment formed is <code>&quot;10&quot;</code>.</li>
	<li>At index 1, <code>nums1[1] = 2</code> and <code>nums0[1] = 0</code>, so the segment formed is <code>&quot;11&quot;</code>.</li>
	<li>Reordering the segments as <code>&quot;11&quot;</code> followed by <code>&quot;10&quot;</code> produces the binary string <code>&quot;1110&quot;</code>.</li>
	<li>The binary number <code>&quot;1110&quot;</code> has value 14 which is the maximum possible value.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums1 = [3,1], nums0 = [0,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">120</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>At index 0, <code>nums1[0] = 3</code> and <code>nums0[0] = 0</code>, so the segment formed is <code>&quot;111&quot;</code>.</li>
	<li>At index 1, <code>nums1[1] = 1</code> and <code>nums0[1] = 3</code>, so the segment formed is <code>&quot;1000&quot;</code>.</li>
	<li>Reordering the segments as <code>&quot;111&quot;</code> followed by <code>&quot;1000&quot;</code> produces the binary string <code>&quot;1111000&quot;</code>.</li>
	<li>The binary number <code>&quot;1111000&quot;</code> has value 120 which is the maximum possible value.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums1.length == nums0.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums1[i], nums0[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>nums1[i] + nums0[i] &gt; 0</code></li>
	<li>The total sum of all elements in <code>nums1</code> and <code>nums0</code> does not exceed 2 * 10<sup>5</sup>.</li>
</ul>


## Hints

1. It is optimal for the segments with more leading ones to come first.
2. Sort the segments by the number of ones in their prefix (in descending order).
3. Start with segments that contain the most ones first.

## Solution

```rust
impl Solution {
    pub fn max_value(black_1: Vec<i32>, black_0: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        let mut black_segments: Vec<(i32, i32)> = black_1.into_iter().zip(black_0.into_iter()).collect();
        black_segments.sort_unstable_by(|&(o1, z1), &(o2, z2)| {
            let s1 = [(1, o1), (0, z1), (1, o2), (0, z2)];
            let s2 = [(1, o2), (0, z2), (1, o1), (0, z1)];
            let (mut i1, mut i2, mut r1, mut r2) = (0, 0, s1[0].1, s2[0].1);
            loop {
                while i1 < 4 && r1 == 0 { i1 += 1; if i1 < 4 { r1 = s1[i1].1; } }
                while i2 < 4 && r2 == 0 { i2 += 1; if i2 < 4 { r2 = s2[i2].1; } }
                if i1 == 4 && i2 == 4 { return Ordering::Equal; }
                if s1[i1].0 != s2[i2].0 { return s2[i2].0.cmp(&s1[i1].0); }
                let take = r1.min(r2);
                r1 -= take; r2 -= take;
            }
        });
        let black_mod = 1_000_000_007i64;
        let mut black_p2 = vec![1i64; 200_005];
        for i in 1..200_005 { black_p2[i] = (black_p2[i - 1] * 2) % black_mod; }
        let mut black_res = 0i64;
        for &(o, z) in &black_segments {
            black_res = (black_res * black_p2[o as usize]) % black_mod;
            black_res = (black_res + (black_p2[o as usize] - 1 + black_mod)) % black_mod;
            black_res = (black_res * black_p2[z as usize]) % black_mod;
        }
        black_res as i32
    }
}
```