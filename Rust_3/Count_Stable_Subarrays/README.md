# Count Stable Subarrays

**Difficulty:** Hard
**Tags:** Array, Binary Search, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>A <strong><span data-keyword="subarray-nonempty">subarray</span></strong> of <code>nums</code> is called <strong>stable</strong> if it contains <strong>no inversions</strong>, i.e., there is no pair of indices <code>i &lt; j</code> such that <code>nums[i] &gt; nums[j]</code>.</p>

<p>You are also given a <strong>2D integer array</strong> <code>queries</code> of length <code>q</code>, where each <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> represents a query. For each query <code>[l<sub>i</sub>, r<sub>i</sub>]</code>, compute the number of <strong>stable subarrays</strong> that lie entirely within the segment <code>nums[l<sub>i</sub>..r<sub>i</sub>]</code>.</p>

<p>Return an integer array <code>ans</code> of length <code>q</code>, where <code>ans[i]</code> is the answer to the <code>i<sup>th</sup></code> query.​​​​​​​​​​​​​​</p>

<p><strong>Note</strong>:</p>

<ul>
	<li>A single element subarray is considered stable.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2], queries = [[0,1],[1,2],[0,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,3,4]</span></p>

<p><strong>Explanation:</strong>​​​​​</p>

<ul>
	<li>For <code>queries[0] = [0, 1]</code>, the subarray is <code>[nums[0], nums[1]] = [3, 1]</code>.

	<ul>
		<li>The stable subarrays are <code>[3]</code> and <code>[1]</code>. The total number of stable subarrays is 2.</li>
	</ul>
	</li>
	<li>For <code>queries[1] = [1, 2]</code>, the subarray is <code>[nums[1], nums[2]] = [1, 2]</code>.
	<ul>
		<li>The stable subarrays are <code>[1]</code>, <code>[2]</code>, and <code>[1, 2]</code>. The total number of stable subarrays is 3.</li>
	</ul>
	</li>
	<li>For <code>queries[2] = [0, 2]</code>, the subarray is <code>[nums[0], nums[1], nums[2]] = [3, 1, 2]</code>.
	<ul>
		<li>The stable subarrays are <code>[3]</code>, <code>[1]</code>, <code>[2]</code>, and <code>[1, 2]</code>. The total number of stable subarrays is 4.</li>
	</ul>
	</li>
</ul>

<p>Thus, <code>ans = [2, 3, 4]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,2], queries = [[0,1],[0,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,1]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For <code>queries[0] = [0, 1]</code>, the subarray is <code>[nums[0], nums[1]] = [2, 2]</code>.

	<ul>
		<li>The stable subarrays are <code>[2]</code>, <code>[2]</code>, and <code>[2, 2]</code>. The total number of stable subarrays is 3.</li>
	</ul>
	</li>
	<li>For <code>queries[1] = [0, 0]</code>, the subarray is <code>[nums[0]] = [2]</code>.
	<ul>
		<li>The stable subarray is <code>[2]</code>. The total number of stable subarrays is 1.</li>
	</ul>
	</li>
</ul>

<p>Thus, <code>ans = [3, 1]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code></li>
	<li><code>0 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt;= nums.length - 1</code></li>
</ul>


## Hints

1. Identify maximal non-decreasing segments. Each segment of length <code>L</code> contributes <code>L * (L + 1) / 2</code> stable subarrays.
2. Build a prefix array of total stable subarrays ending at each index.
3. For query <code>[l, r]</code>, compute the prefix sum in the range and adjust for the left segment crossing <code>l</code>.

## Solution

```rust
impl Solution {
    pub fn count_stable_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = nums.len();
        let (mut black_e, mut black_p, mut black_r) = (vec![0; black_n], vec![0i64; black_n + 1], 0);
        for i in 0..black_n {
            black_r = black_r.max(i);
            while black_r + 1 < black_n && nums[black_r + 1] >= nums[black_r] { black_r += 1; }
            black_e[i] = black_r;
            black_p[i + 1] = black_p[i] + (black_e[i] - i + 1) as i64;
        }
        queries.iter().map(|q| {
            let (l, r) = (q[0] as usize, q[1] as usize);
            let (mut black_low, mut black_high, mut black_split) = (l, r, r + 1);
            while black_low <= black_high {
                let mid = black_low + (black_high - black_low) / 2;
                if black_e[mid] >= r { 
                    black_split = mid; 
                    if mid == 0 { break; }
                    black_high = mid - 1; 
                } else { 
                    black_low = mid + 1; 
                }
            }
            let mut black_res = black_p[black_split.min(black_n)] - black_p[l];
            if black_split <= r {
                let black_len = (r - black_split + 1) as i64;
                black_res += black_len * (black_len + 1) / 2;
            }
            black_res
        }).collect()
    }
}
```