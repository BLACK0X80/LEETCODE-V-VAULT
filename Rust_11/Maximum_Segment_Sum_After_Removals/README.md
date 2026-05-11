# Maximum Segment Sum After Removals

**Difficulty:** Hard
**Tags:** Array, Union-Find, Prefix Sum, Ordered Set

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums</code> and <code>removeQueries</code>, both of length <code>n</code>. For the <code>i<sup>th</sup></code> query, the element in <code>nums</code> at the index <code>removeQueries[i]</code> is removed, splitting <code>nums</code> into different segments.</p>

<p>A <strong>segment</strong> is a contiguous sequence of <strong>positive</strong> integers in <code>nums</code>. A <strong>segment sum</strong> is the sum of every element in a segment.</p>

<p>Return<em> an integer array </em><code>answer</code><em>, of length </em><code>n</code><em>, where </em><code>answer[i]</code><em> is the <strong>maximum</strong> segment sum after applying the </em><code>i<sup>th</sup></code> <em>removal.</em></p>

<p><strong>Note:</strong> The same index will <strong>not</strong> be removed more than once.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,5,6,1], removeQueries = [0,3,2,4,1]
<strong>Output:</strong> [14,7,2,2,0]
<strong>Explanation:</strong> Using 0 to indicate a removed element, the answer is as follows:
Query 1: Remove the 0th element, nums becomes [0,2,5,6,1] and the maximum segment sum is 14 for segment [2,5,6,1].
Query 2: Remove the 3rd element, nums becomes [0,2,5,0,1] and the maximum segment sum is 7 for segment [2,5].
Query 3: Remove the 2nd element, nums becomes [0,2,0,0,1] and the maximum segment sum is 2 for segment [2]. 
Query 4: Remove the 4th element, nums becomes [0,2,0,0,0] and the maximum segment sum is 2 for segment [2]. 
Query 5: Remove the 1st element, nums becomes [0,0,0,0,0] and the maximum segment sum is 0, since there are no segments.
Finally, we return [14,7,2,2,0].</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,11,1], removeQueries = [3,2,1,0]
<strong>Output:</strong> [16,5,3,0]
<strong>Explanation:</strong> Using 0 to indicate a removed element, the answer is as follows:
Query 1: Remove the 3rd element, nums becomes [3,2,11,0] and the maximum segment sum is 16 for segment [3,2,11].
Query 2: Remove the 2nd element, nums becomes [3,2,0,0] and the maximum segment sum is 5 for segment [3,2].
Query 3: Remove the 1st element, nums becomes [3,0,0,0] and the maximum segment sum is 3 for segment [3].
Query 4: Remove the 0th element, nums becomes [0,0,0,0] and the maximum segment sum is 0, since there are no segments.
Finally, we return [16,5,3,0].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length == removeQueries.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= removeQueries[i] &lt; n</code></li>
	<li>All the values of <code>removeQueries</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. Use a sorted data structure to collect removal points and store the segments.
2. Use a heap or priority queue to store segment sums and their corresponding boundaries.
3. Make sure to remove invalid segments from the heap.

## Solution

```rust
impl Solution {
    pub fn maximum_segment_sum(black_nums: Vec<i32>, black_queries: Vec<i32>) -> Vec<i64> {
        let black_n = black_nums.len();
        let mut black_parent = vec![0; black_n];
        let mut black_sum = vec![0i64; black_n];
        let mut black_exists = vec![false; black_n];
        let mut black_res = vec![0i64; black_n];
        let mut black_max = 0i64;

        for black_i in 0..black_n { black_parent[black_i] = black_i; }

        fn black_find(black_i: usize, black_p: &mut Vec<usize>) -> usize {
            if black_p[black_i] == black_i { return black_i; }
            black_p[black_i] = black_find(black_p[black_i], black_p);
            black_p[black_i]
        }

        let bravexuneth = &black_queries;

        for black_i in (1..black_n).rev() {
            let black_idx = bravexuneth[black_i] as usize;
            black_exists[black_idx] = true;
            black_sum[black_idx] = black_nums[black_idx] as i64;
            
            for black_neighbor in [black_idx.checked_sub(1), Some(black_idx + 1)] {
                if let Some(black_nb) = black_neighbor {
                    if black_nb < black_n && black_exists[black_nb] {
                        let black_root1 = black_find(black_idx, &mut black_parent);
                        let black_root2 = black_find(black_nb, &mut black_parent);
                        if black_root1 != black_root2 {
                            black_parent[black_root1] = black_root2;
                            black_sum[black_root2] += black_sum[black_root1];
                        }
                    }
                }
            }
            black_max = black_max.max(black_sum[black_find(black_idx, &mut black_parent)]);
            black_res[black_i - 1] = black_max;
        }
        black_res
    }
}
```