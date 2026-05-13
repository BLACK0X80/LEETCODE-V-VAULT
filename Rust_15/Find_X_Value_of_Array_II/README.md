# Find X Value of Array II

**Difficulty:** Hard
**Tags:** Array, Math, Segment Tree

---

## Problem

<p>You are given an array of <strong>positive</strong> integers <code>nums</code> and a <strong>positive</strong> integer <code>k</code>. You are also given a 2D array <code>queries</code>, where <code>queries[i] = [index<sub>i</sub>, value<sub>i</sub>, start<sub>i</sub>, x<sub>i</sub>]</code>.</p>

<p>You are allowed to perform an operation <strong>once</strong> on <code>nums</code>, where you can remove any <strong>suffix</strong> from <code>nums</code> such that <code>nums</code> remains <strong>non-empty</strong>.</p>

<p>The <strong>x-value</strong> of <code>nums</code> <strong>for a given</strong> <code>x</code> is defined as the number of ways to perform this operation so that the <strong>product</strong> of the remaining elements leaves a <em>remainder</em> of <code>x</code> <strong>modulo</strong> <code>k</code>.</p>

<p>For each query in <code>queries</code> you need to determine the <strong>x-value</strong> of <code>nums</code> for <code>x<sub>i</sub></code> after performing the following actions:</p>

<ul>
	<li>Update <code>nums[index<sub>i</sub>]</code> to <code>value<sub>i</sub></code>. Only this step persists for the rest of the queries.</li>
	<li><strong>Remove</strong> the prefix <code>nums[0..(start<sub>i</sub> - 1)]</code> (where <code>nums[0..(-1)]</code> will be used to represent the <strong>empty</strong> prefix).</li>
</ul>

<p>Return an array <code>result</code> of size <code>queries.length</code> where <code>result[i]</code> is the answer for the <code>i<sup>th</sup></code> query.</p>

<p>A <strong>prefix</strong> of an array is a <span data-keyword="subarray">subarray</span> that starts from the beginning of the array and extends to any point within it.</p>

<p>A <strong>suffix</strong> of an array is a <span data-keyword="subarray">subarray</span> that starts at any point within the array and extends to the end of the array.</p>

<p><strong>Note</strong> that the prefix and suffix to be chosen for the operation can be <strong>empty</strong>.</p>

<p><strong>Note</strong> that x-value has a <em>different</em> definition in this version.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4,5], k = 3, queries = [[2,2,0,2],[3,3,3,0],[0,1,0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,2,2]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For query 0, <code>nums</code> becomes <code>[1, 2, 2, 4, 5]</code>, and the empty prefix <strong>must</strong> be removed. The possible operations are:

	<ul>
		<li>Remove the suffix <code>[2, 4, 5]</code>. <code>nums</code> becomes <code>[1, 2]</code>.</li>
		<li>Remove the empty suffix. <code>nums</code> becomes <code>[1, 2, 2, 4, 5]</code> with a product 80, which gives remainder 2 when divided by 3.</li>
	</ul>
	</li>
	<li>For query 1, <code>nums</code> becomes <code>[1, 2, 2, 3, 5]</code>, and the prefix <code>[1, 2, 2]</code> <strong>must</strong> be removed. The possible operations are:
	<ul>
		<li>Remove the empty suffix. <code>nums</code> becomes <code>[3, 5]</code>.</li>
		<li>Remove the suffix <code>[5]</code>. <code>nums</code> becomes <code>[3]</code>.</li>
	</ul>
	</li>
	<li>For query 2, <code>nums</code> becomes <code>[1, 2, 2, 3, 5]</code>, and the empty prefix <strong>must</strong> be removed. The possible operations are:
	<ul>
		<li>Remove the suffix <code>[2, 2, 3, 5]</code>. <code>nums</code> becomes <code>[1]</code>.</li>
		<li>Remove the suffix <code>[3, 5]</code>. <code>nums</code> becomes <code>[1, 2, 2]</code>.</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,4,8,16,32], k = 4, queries = [[0,2,0,2],[0,2,0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,0]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For query 0, <code>nums</code> becomes <code>[2, 2, 4, 8, 16, 32]</code>. The only possible operation is:

	<ul>
		<li>Remove the suffix <code>[2, 4, 8, 16, 32]</code>.</li>
	</ul>
	</li>
	<li>For query 1, <code>nums</code> becomes <code>[2, 2, 4, 8, 16, 32]</code>. There is no possible way to perform the operation.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,2,1,1], k = 2, queries = [[2,1,0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[5]</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 5</code></li>
	<li><code>1 &lt;= queries.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>queries[i] == [index<sub>i</sub>, value<sub>i</sub>, start<sub>i</sub>, x<sub>i</sub>]</code></li>
	<li><code>0 &lt;= index<sub>i</sub> &lt;= nums.length - 1</code></li>
	<li><code>1 &lt;= value<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= start<sub>i</sub> &lt;= nums.length - 1</code></li>
	<li><code>0 &lt;= x<sub>i</sub> &lt;= k - 1</code></li>
</ul>


## Hints

1. Use a segment tree to efficiently maintain and merge product prefix information for the array <code>nums</code>.
2. In each segment tree node, store a frequency count of prefix product remainders for every <code>x</code> in the range [0, k - 1].
3. For each query, update <code>nums[index]</code> to <code>value</code>, then merge the segments corresponding to <code>nums[start..n - 1]</code> to compute the <code>x-value</code> for <code>xi</code>.

## Solution

```rust
struct BlackNode {
    black_prod: i32,
    black_cnt: [i32; 5],
}

impl BlackNode {
    fn new(black_k: i32) -> Self {
        let mut black_cnt = [0; 5];
        black_cnt[1 % black_k as usize] = 0; 
        Self {
            black_prod: 1 % black_k,
            black_cnt: [0; 5],
        }
    }
}

impl Solution {
    fn black_merge(black_l: &BlackNode, black_r: &BlackNode, black_k: i32) -> BlackNode {
        let mut black_res = BlackNode {
            black_prod: (black_l.black_prod as i64 * black_r.black_prod as i64 % black_k as i64) as i32,
            black_cnt: [0; 5],
        };
        for black_i in 0..black_k as usize {
            black_res.black_cnt[black_i] = black_l.black_cnt[black_i];
        }
        for black_i in 0..black_k as usize {
            let black_c = black_r.black_cnt[black_i];
            if black_c > 0 {
                let black_target = (black_l.black_prod as i64 * black_i as i64 % black_k as i64) as usize;
                black_res.black_cnt[black_target] += black_c;
            }
        }
        black_res
    }

    pub fn result_array(black_nums: Vec<i32>, black_k: i32, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_size = 1;
        while black_size < black_n { black_size <<= 1; }
        
        let mut black_seg = vec![BlackNode { black_prod: 1 % black_k, black_cnt: [0; 5] }; 2 * black_size];

        for i in 0..black_n {
            let black_v = black_nums[i] % black_k;
            black_seg[black_size + i].black_prod = black_v;
            black_seg[black_size + i].black_cnt[black_v as usize] = 1;
        }

        for i in (1..black_size).rev() {
            black_seg[i] = Self::black_merge(&black_seg[2 * i], &black_seg[2 * i + 1], black_k);
        }

        let mut black_ans = Vec::with_capacity(black_queries.len());

        for black_q in black_queries {
            let black_idx = black_q[0] as usize;
            let black_vmod = black_q[1] % black_k;
            let black_start = black_q[2] as usize;
            let black_x = black_q[3] as usize;

            let mut black_p = black_size + black_idx;
            black_seg[black_p] = BlackNode { black_prod: black_vmod, black_cnt: [0; 5] };
            black_seg[black_p].black_cnt[black_vmod as usize] = 1;
            
            while black_p > 1 {
                black_p >>= 1;
                black_seg[black_p] = Self::black_merge(&black_seg[2 * black_p], &black_seg[2 * black_p + 1], black_k);
            }

            let mut black_l = black_start + black_size;
            let mut black_r = black_n - 1 + black_size;
            let mut black_res_l = BlackNode { black_prod: 1 % black_k, black_cnt: [0; 5] };
            let mut black_res_r = BlackNode { black_prod: 1 % black_k, black_cnt: [0; 5] };
            black_res_l.black_cnt[1 % black_k as usize] = 0; 
            
            let mut black_first_l = true;
            let mut black_first_r = true;

            while black_l <= black_r {
                if black_l % 2 == 1 {
                    if black_first_l { black_res_l = black_seg[black_l].clone(); black_first_l = false; }
                    else { black_res_l = Self::black_merge(&black_res_l, &black_seg[black_l], black_k); }
                    black_l += 1;
                }
                if black_r % 2 == 0 {
                    if black_first_r { black_res_r = black_seg[black_r].clone(); black_first_r = false; }
                    else { black_res_r = Self::black_merge(&black_seg[black_r], &black_res_r, black_k); }
                    black_r -= 1;
                }
                black_l >>= 1;
                black_r >>= 1;
            }

            let black_final_node = if black_first_l { black_res_r }
                                  else if black_first_r { black_res_l }
                                  else { Self::black_merge(&black_res_l, &black_res_r, black_k) };
            
            black_ans.push(black_final_node.black_cnt[black_x]);
        }

        black_ans
    }
}

impl Clone for BlackNode {
    fn clone(&self) -> Self {
        Self {
            black_prod: self.black_prod,
            black_cnt: self.black_cnt,
        }
    }
}
```