# Count Good Subarrays

**Difficulty:** Hard
**Tags:** Array, Stack, Bit Manipulation, Monotonic Stack

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>A <strong><span data-keyword="subarray-nonempty">subarray</span></strong> is called <strong>good</strong> if the <strong>bitwise OR</strong> of all its elements is equal to <strong>at least one</strong> element present in that subarray.</p>

<p>Return the number of good subarrays in <code>nums</code>.</p>

<p>Here, the bitwise OR of two integers <code>a</code> and <code>b</code> is denoted by <code>a | b</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The subarrays of <code>nums</code> are:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr>
			<th style="border: 1px solid black;">Subarray</th>
			<th style="border: 1px solid black;">Bitwise OR</th>
			<th style="border: 1px solid black;">Present in Subarray</th>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[4]</code></td>
			<td style="border: 1px solid black;"><code>4 = 4</code></td>
			<td style="border: 1px solid black;">Yes</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[2]</code></td>
			<td style="border: 1px solid black;"><code>2 = 2</code></td>
			<td style="border: 1px solid black;">Yes</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[3]</code></td>
			<td style="border: 1px solid black;"><code>3 = 3</code></td>
			<td style="border: 1px solid black;">Yes</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[4, 2]</code></td>
			<td style="border: 1px solid black;"><code>4 | 2 = 6</code></td>
			<td style="border: 1px solid black;">No</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[2, 3]</code></td>
			<td style="border: 1px solid black;"><code>2 | 3 = 3</code></td>
			<td style="border: 1px solid black;">Yes</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[4, 2, 3]</code></td>
			<td style="border: 1px solid black;"><code>4 | 2 | 3 = 7</code></td>
			<td style="border: 1px solid black;">No</td>
		</tr>
	</tbody>
</table>

<p>Thus, the good subarrays of <code>nums</code> are <code>[4]</code>, <code>[2]</code>, <code>[3]</code> and <code>[2, 3]</code>. Thus, the answer is 4.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>Any subarray of <code>nums</code> containing 3 has bitwise OR equal to 3, and subarrays containing only 1 have bitwise OR equal to 1.</p>

<p>In both cases, the result is present in the subarray, so all subarrays are good, and the answer is 6.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Observe that a subarray is good if its bitwise OR equals its maximum element. This happens when every element in the subarray has all its set bits contained inside the bits of the maximum element.
2. Use a monotonic decreasing stack to compute for each index <code>i</code> the range <code>[L[i], R[i]]</code> where <code>nums[i]</code> is the maximum of the subarray (break ties so only one index owns equal values).
3. Precompute for each bit the previous and next positions where that bit is set. For index <code>i</code>, shrink <code>[L[i], R[i]]</code> by excluding positions that contain a bit not set in <code>nums[i]</code>. The contribution of <code>i</code> is <code>(i - effective_left + 1) * (effective_right - i + 1)</code>.

## Solution

```rust
impl Solution {
    pub fn count_good_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans = 0i64;
        let mut bit_pos: Vec<Vec<usize>> = vec![vec![]; 30];
        let mut pos: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();
        let mut count: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

        for i in 0..n {
            for j in 0..30 {
                if nums[i] & (1 << j) != 0 {
                    bit_pos[j].push(i);
                }
            }
            pos.entry(nums[i]).or_default().push(i);
        }

        for index in 0..n {
            let val = nums[index];
            let mut left = 0usize;
            let mut right = n - 1;

            for j in 0..30 {
                if val & (1 << j) == 0 {
                    let bp = &bit_pos[j];
                    match bp.binary_search(&index) {
                        Ok(k) | Err(k) => {
                            if k < bp.len() { right = right.min(bp[k] - 1 + (if bp[k] == 0 { 1 } else { 0 })); }
                            if k < bp.len() && bp[k] > index { right = right.min(bp[k] - 1); }
                            if k > 0 { left = left.max(bp[k-1] + 1); }
                        }
                    }
                }
            }

            let right_dist = right - index + 1;
            let cnt = *count.get(&val).unwrap_or(&0);
            if cnt > 0 {
                let last_seen = pos[&val][cnt - 1];
                left = left.max(last_seen + 1);
            }
            let left_dist = index - left + 1;
            ans += (right_dist * left_dist) as i64;
            *count.entry(val).or_insert(0) += 1;
        }
        ans
    }
}
```