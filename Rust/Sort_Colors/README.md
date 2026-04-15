# Sort Colors

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Sorting

---

## Problem

<p>Given an array <code>nums</code> with <code>n</code> objects colored red, white, or blue, sort them <strong><a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> </strong>so that objects of the same color are adjacent, with the colors in the order red, white, and blue.</p>

<p>We will use the integers <code>0</code>, <code>1</code>, and <code>2</code> to represent the color red, white, and blue, respectively.</p>

<p>You must solve this problem without using the library&#39;s sort function.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,0,2,1,1,0]
<strong>Output:</strong> [0,0,1,1,2,2]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,0,1]
<strong>Output:</strong> [0,1,2]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>1 &lt;= n &lt;= 300</code></li>
	<li><code>nums[i]</code> is either <code>0</code>, <code>1</code>, or <code>2</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong>&nbsp;Could you come up with a one-pass algorithm using only&nbsp;constant extra space?</p>


## Hints

1. A rather straight forward solution is a two-pass algorithm using counting sort.
2. Iterate the array counting number of 0's, 1's, and 2's.
3. Overwrite array with the total number of 0's, then 1's and followed by 2's.

## Solution

```rust
impl Solution {
    pub fn sort_colors(black_n: &mut Vec<i32>) {
        let (mut black_l, mut black_curr, mut black_r) = (0, 0, black_n.len() as i32 - 1);
        while black_curr <= black_r {
            if black_n[black_curr as usize] == 0 { black_n.swap(black_l, black_curr as usize); black_l += 1; black_curr += 1; }
            else if black_n[black_curr as usize] == 2 { black_n.swap(black_curr as usize, black_r as usize); black_r -= 1; }
            else { black_curr += 1; }
        }
    }
}
```