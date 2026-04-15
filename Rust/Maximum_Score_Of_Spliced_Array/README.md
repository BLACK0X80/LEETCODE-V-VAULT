# Maximum Score Of Spliced Array

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code>, both of length <code>n</code>.</p>

<p>You can choose two integers <code>left</code> and <code>right</code> where <code>0 &lt;= left &lt;= right &lt; n</code> and <strong>swap</strong> the subarray <code>nums1[left...right]</code> with the subarray <code>nums2[left...right]</code>.</p>

<ul>
	<li>For example, if <code>nums1 = [1,2,3,4,5]</code> and <code>nums2 = [11,12,13,14,15]</code> and you choose <code>left = 1</code> and <code>right = 2</code>, <code>nums1</code> becomes <code>[1,<strong><u>12,13</u></strong>,4,5]</code> and <code>nums2</code> becomes <code>[11,<strong><u>2,3</u></strong>,14,15]</code>.</li>
</ul>

<p>You may choose to apply the mentioned operation <strong>once</strong> or not do anything.</p>

<p>The <strong>score</strong> of the arrays is the <strong>maximum</strong> of <code>sum(nums1)</code> and <code>sum(nums2)</code>, where <code>sum(arr)</code> is the sum of all the elements in the array <code>arr</code>.</p>

<p>Return <em>the <strong>maximum possible score</strong></em>.</p>

<p>A <strong>subarray</strong> is a contiguous sequence of elements within an array. <code>arr[left...right]</code> denotes the subarray that contains the elements of <code>nums</code> between indices <code>left</code> and <code>right</code> (<strong>inclusive</strong>).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [60,60,60], nums2 = [10,90,10]
<strong>Output:</strong> 210
<strong>Explanation:</strong> Choosing left = 1 and right = 1, we have nums1 = [60,<u><strong>90</strong></u>,60] and nums2 = [10,<u><strong>60</strong></u>,10].
The score is max(sum(nums1), sum(nums2)) = max(210, 80) = 210.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [20,40,20,70,30], nums2 = [50,20,50,40,20]
<strong>Output:</strong> 220
<strong>Explanation:</strong> Choosing left = 3, right = 4, we have nums1 = [20,40,20,<u><strong>40,20</strong></u>] and nums2 = [50,20,50,<u><strong>70,30</strong></u>].
The score is max(sum(nums1), sum(nums2)) = max(140, 220) = 220.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [7,11,13], nums2 = [1,1,1]
<strong>Output:</strong> 31
<strong>Explanation:</strong> We choose not to swap any subarray.
The score is max(sum(nums1), sum(nums2)) = max(31, 3) = 31.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums1.length == nums2.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums1[i], nums2[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Think on Dynamic Programming.
2. First assume you will be taking the array a and choose some subarray from b
3. Suppose the DP is DP(pos, state). pos is the current position you are in. state is one of {0,1,2}, where 0 means taking the array a, 1 means we are taking the subarray b, and 2 means we are again taking the array a. We need to handle the transitions carefully.

## Solution

```rust
impl Solution {
    pub fn maximums_spliced_array(black_n1: Vec<i32>, black_n2: Vec<i32>) -> i32 {
        fn black_solve(black_a: &Vec<i32>, black_b: &Vec<i32>) -> i32 {
            let mut black_max_delta = 0;
            let mut black_curr_delta = 0;
            for black_i in 0..black_a.len() {
                black_curr_delta += black_b[black_i] - black_a[black_i];
                if black_curr_delta < 0 { black_curr_delta = 0; }
                black_max_delta = black_max_delta.max(black_curr_delta);
            }
            black_a.iter().sum::<i32>() + black_max_delta
        }

        let bravexuneth = &black_n2;
        black_solve(&black_n1, bravexuneth).max(black_solve(bravexuneth, &black_n1))
    }
}
```