# Count of Range Sum

**Difficulty:** Hard
**Tags:** Array, Binary Search, Divide and Conquer, Binary Indexed Tree, Segment Tree, Merge Sort, Ordered Set

---

## Problem

<p>Given an integer array <code>nums</code> and two integers <code>lower</code> and <code>upper</code>, return <em>the number of range sums that lie in</em> <code>[lower, upper]</code> <em>inclusive</em>.</p>

<p>Range sum <code>S(i, j)</code> is defined as the sum of the elements in <code>nums</code> between indices <code>i</code> and <code>j</code> inclusive, where <code>i &lt;= j</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [-2,5,-1], lower = -2, upper = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> The three ranges are: [0,0], [2,2], and [0,2] and their respective sums are: -2, -1, 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0], lower = 0, upper = 0
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-2<sup>31</sup> &lt;= nums[i] &lt;= 2<sup>31</sup> - 1</code></li>
	<li><code>-10<sup>5</sup> &lt;= lower &lt;= upper &lt;= 10<sup>5</sup></code></li>
	<li>The answer is <strong>guaranteed</strong> to fit in a <strong>32-bit</strong> integer.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let lower = lower as i64;
        let upper = upper as i64;

        let mut prefix = vec![0i64; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        merge_count(&mut prefix, lower, upper)
    }
}

fn merge_count(arr: &mut [i64], lower: i64, upper: i64) -> i32 {
    if arr.len() <= 1 {
        return 0;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    let mut count = merge_count(left, lower, upper)
                  + merge_count(right, lower, upper);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut lo = 0usize;
    let mut hi = 0usize;
    let mut j = 0usize;

    for &l in &left {
        while lo < right.len() && right[lo] - l < lower { lo += 1; }
        while hi < right.len() && right[hi] - l <= upper { hi += 1; }
        count += (hi - lo) as i32;
    }

    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    arr.copy_from_slice(&merged);

    count
}
```