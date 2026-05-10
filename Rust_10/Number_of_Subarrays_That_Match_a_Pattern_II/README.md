# Number of Subarrays That Match a Pattern II

**Difficulty:** Hard
**Tags:** Array, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> of size <code>n</code>, and a <strong>0-indexed</strong> integer array <code>pattern</code> of size <code>m</code> consisting of integers <code>-1</code>, <code>0</code>, and <code>1</code>.</p>

<p>A <span data-keyword="subarray">subarray</span> <code>nums[i..j]</code> of size <code>m + 1</code> is said to match the <code>pattern</code> if the following conditions hold for each element <code>pattern[k]</code>:</p>

<ul>
	<li><code>nums[i + k + 1] &gt; nums[i + k]</code> if <code>pattern[k] == 1</code>.</li>
	<li><code>nums[i + k + 1] == nums[i + k]</code> if <code>pattern[k] == 0</code>.</li>
	<li><code>nums[i + k + 1] &lt; nums[i + k]</code> if <code>pattern[k] == -1</code>.</li>
</ul>

<p>Return <em>the<strong> count</strong> of subarrays in</em> <code>nums</code> <em>that match the</em> <code>pattern</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6], pattern = [1,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The pattern [1,1] indicates that we are looking for strictly increasing subarrays of size 3. In the array nums, the subarrays [1,2,3], [2,3,4], [3,4,5], and [4,5,6] match this pattern.
Hence, there are 4 subarrays in nums that match the pattern.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,4,4,1,3,5,5,3], pattern = [1,0,-1]
<strong>Output:</strong> 2
<strong>Explanation: </strong>Here, the pattern [1,0,-1] indicates that we are looking for a sequence where the first number is smaller than the second, the second is equal to the third, and the third is greater than the fourth. In the array nums, the subarrays [1,4,4,1], and [3,5,5,3] match this pattern.
Hence, there are 2 subarrays in nums that match the pattern.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums.length &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= m == pattern.length &lt; n</code></li>
	<li><code>-1 &lt;= pattern[i] &lt;= 1</code></li>
</ul>


## Hints

1. Create a second array <code>nums2</code> such that <code>nums2[i] = 1</code> if <code>nums[i + 1] > nums[i]</code>, <code>nums2[i] = 0</code> if <code>nums[i + 1] == nums[i]</code>, and <code>nums2[i] = -1</code> if <code>nums[i + 1] < nums[i]</code>.
2. The problem becomes: “Count the number of subarrays in <code>nums2</code> that are equal to <code>pattern</code>.
3. Use Knuth-Morris-Pratt or Z-Function algorithms.

## Solution

```rust
impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = pattern.len();
        let text: Vec<i32> = (1..n).map(|i| nums[i].cmp(&nums[i-1]) as i32).collect();
        let mut fail = vec![0usize; m];
        let mut j = 0;
        for i in 1..m {
            while j > 0 && pattern[i] != pattern[j] { j = fail[j-1]; }
            if pattern[i] == pattern[j] { j += 1; }
            fail[i] = j;
        }
        let mut count = 0;
        j = 0;
        for i in 0..text.len() {
            while j > 0 && text[i] != pattern[j] { j = fail[j-1]; }
            if text[i] == pattern[j] { j += 1; }
            if j == m { count += 1; j = fail[j-1]; }
        }
        count
    }
}
```