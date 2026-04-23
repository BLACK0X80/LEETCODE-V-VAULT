# Reverse Pairs

**Difficulty:** Hard
**Tags:** Array, Binary Search, Divide and Conquer, Binary Indexed Tree, Segment Tree, Merge Sort, Ordered Set

---

## Problem

<p>Given an integer array <code>nums</code>, return <em>the number of <strong>reverse pairs</strong> in the array</em>.</p>

<p>A <strong>reverse pair</strong> is a pair <code>(i, j)</code> where:</p>

<ul>
	<li><code>0 &lt;= i &lt; j &lt; nums.length</code> and</li>
	<li><code>nums[i] &gt; 2 * nums[j]</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,2,3,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The reverse pairs are:
(1, 4) --&gt; nums[1] = 3, nums[4] = 1, 3 &gt; 2 * 1
(3, 4) --&gt; nums[3] = 3, nums[4] = 1, 3 &gt; 2 * 1
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,4,3,5,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The reverse pairs are:
(1, 4) --&gt; nums[1] = 4, nums[4] = 1, 4 &gt; 2 * 1
(2, 4) --&gt; nums[2] = 3, nums[4] = 1, 3 &gt; 2 * 1
(3, 4) --&gt; nums[3] = 5, nums[4] = 1, 5 &gt; 2 * 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>-2<sup>31</sup> &lt;= nums[i] &lt;= 2<sup>31</sup> - 1</code></li>
</ul>


## Hints

1. Use the merge-sort technique.
2. Divide the array into two parts and sort them.
3. For each integer in the first part, count the number of integers that satisfy the condition from the second part. Use the pointer to help you in the counting process.

## Solution

```rust
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        let mut temp = vec![0i64; n];
        Self::merge_sort(&mut nums, &mut temp, 0, n) as i32
    }
    fn merge_sort(nums: &mut [i32], temp: &mut [i64], l: usize, r: usize) -> usize {
        if r - l <= 1 { return 0; }
        let m = l + (r - l) / 2;
        let mut cnt = Self::merge_sort(nums, temp, l, m) + Self::merge_sort(nums, temp, m, r);
        let mut j = m;
        for i in l..m {
            while j < r && nums[i] as i64 > 2 * nums[j] as i64 { j += 1; }
            cnt += j - m;
        }
        for i in l..r { temp[i] = nums[i] as i64; }
        let (mut i, mut k) = (l, l);
        j = m;
        while i < m && j < r {
            if temp[i] <= temp[j] { nums[k] = temp[i] as i32; i += 1; }
            else { nums[k] = temp[j] as i32; j += 1; }
            k += 1;
        }
        while i < m { nums[k] = temp[i] as i32; i += 1; k += 1; }
        while j < r { nums[k] = temp[j] as i32; j += 1; k += 1; }
        cnt
    }
}
```