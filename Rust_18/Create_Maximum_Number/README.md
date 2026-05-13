# Create Maximum Number

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Stack, Greedy, Monotonic Stack

---

## Problem

<p>You are given two integer arrays <code>nums1</code> and <code>nums2</code> of lengths <code>m</code> and <code>n</code> respectively. <code>nums1</code> and <code>nums2</code> represent the digits of two numbers. You are also given an integer <code>k</code>.</p>

<p>Create the maximum number of length <code>k &lt;= m + n</code> from digits of the two numbers. The relative order of the digits from the same array must be preserved.</p>

<p>Return an array of the <code>k</code> digits representing the answer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
<strong>Output:</strong> [9,8,6,5,3]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [6,7], nums2 = [6,0,4], k = 5
<strong>Output:</strong> [6,7,6,0,4]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [3,9], nums2 = [8,9], k = 3
<strong>Output:</strong> [9,8,9]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == nums1.length</code></li>
	<li><code>n == nums2.length</code></li>
	<li><code>1 &lt;= m, n &lt;= 500</code></li>
	<li><code>0 &lt;= nums1[i], nums2[i] &lt;= 9</code></li>
	<li><code>1 &lt;= k &lt;= m + n</code></li>
	<li><code>nums1</code> and <code>nums2</code> do not have leading zeros.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let m = nums1.len();
        let n = nums2.len();
        let k = k as usize;
        let mut best = vec![0i32; k];

        let lo = if k > n { k - n } else { 0 };
        let hi = k.min(m);

        for i in lo..=hi {
            let j = k - i;
            if j > n { continue; }

            let sub1 = max_subsequence(&nums1, i);
            let sub2 = max_subsequence(&nums2, j);
            let merged = merge(&sub1, &sub2);

            if merged > best {
                best = merged;
            }
        }

        best
    }
}

fn max_subsequence(nums: &[i32], k: usize) -> Vec<i32> {
    let n = nums.len();
    let mut stack: Vec<i32> = Vec::new();
    let mut drop = n - k;

    for &num in nums {
        while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < num {
            stack.pop();
            drop -= 1;
        }
        stack.push(num);
    }

    stack[..k].to_vec()
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i..] >= b[j..] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}
```