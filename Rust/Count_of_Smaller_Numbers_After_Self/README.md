# Count of Smaller Numbers After Self

**Difficulty:** Hard
**Tags:** Array, Binary Search, Divide and Conquer, Binary Indexed Tree, Segment Tree, Merge Sort, Ordered Set

---

## Problem

<p>Given an integer array <code>nums</code>, return<em> an integer array </em><code>counts</code><em> where </em><code>counts[i]</code><em> is the number of smaller elements to the right of </em><code>nums[i]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,2,6,1]
<strong>Output:</strong> [2,1,1,0]
<strong>Explanation:</strong>
To the right of 5 there are <b>2</b> smaller elements (2 and 1).
To the right of 2 there is only <b>1</b> smaller element (1).
To the right of 6 there is <b>1</b> smaller element (1).
To the right of 1 there is <b>0</b> smaller element.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [-1]
<strong>Output:</strong> [0]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [-1,-1]
<strong>Output:</strong> [0,0]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut counts = vec![0i32; n];
        let mut indexed: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        let mut temp = vec![(0i32, 0usize); n];

        merge_sort(&mut indexed, &mut temp, &mut counts, 0, n);
        counts
    }
}

fn merge_sort(
    arr: &mut Vec<(i32, usize)>,
    temp: &mut Vec<(i32, usize)>,
    counts: &mut Vec<i32>,
    left: usize,
    right: usize,
) {
    if right - left <= 1 {
        return;
    }

    let mid = (left + right) / 2;
    merge_sort(arr, temp, counts, left, mid);
    merge_sort(arr, temp, counts, mid, right);

    let mut i = left;
    let mut j = mid;
    let mut k = left;

    while i < mid && j < right {
        if arr[i].0 <= arr[j].0 {
            counts[arr[i].1] += (j - mid) as i32;
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        counts[arr[i].1] += (j - mid) as i32;
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j < right {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }

    arr[left..right].copy_from_slice(&temp[left..right]);
}
```