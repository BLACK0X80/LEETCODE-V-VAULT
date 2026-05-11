# Maximize Subarray Sum After Removing All Occurrences of One Element

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Segment Tree

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>You can do the following operation on the array <strong>at most</strong> once:</p>

<ul>
	<li>Choose <strong>any</strong> integer <code>x</code> such that <code>nums</code> remains <strong>non-empty</strong> on removing all occurrences of <code>x</code>.</li>
	<li>Remove&nbsp;<strong>all</strong> occurrences of <code>x</code> from the array.</li>
</ul>

<p>Return the <strong>maximum</strong> <span data-keyword="subarray-nonempty">subarray</span> sum across <strong>all</strong> possible resulting arrays.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [-3,2,-2,-1,3,-2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<p>We can have the following arrays after at most one operation:</p>

<ul>
	<li>The original array is <code>nums = [<span class="example-io">-3, 2, -2, -1, <u><strong>3, -2, 3</strong></u></span>]</code>. The maximum subarray sum is <code>3 + (-2) + 3 = 4</code>.</li>
	<li>Deleting all occurences of <code>x = -3</code> results in <code>nums = [2, -2, -1, <strong><u><span class="example-io">3, -2, 3</span></u></strong>]</code>. The maximum subarray sum is <code>3 + (-2) + 3 = 4</code>.</li>
	<li>Deleting all occurences of <code>x = -2</code> results in <code>nums = [<span class="example-io">-3, <strong><u>2, -1, 3, 3</u></strong></span>]</code>. The maximum subarray sum is <code>2 + (-1) + 3 + 3 = 7</code>.</li>
	<li>Deleting all occurences of <code>x = -1</code> results in <code>nums = [<span class="example-io">-3, 2, -2, <strong><u>3, -2, 3</u></strong></span>]</code>. The maximum subarray sum is <code>3 + (-2) + 3 = 4</code>.</li>
	<li>Deleting all occurences of <code>x = 3</code> results in <code>nums = [<span class="example-io">-3, <u><strong>2</strong></u>, -2, -1, -2</span>]</code>. The maximum subarray sum is 2.</li>
</ul>

<p>The output is <code>max(4, 4, 7, 4, 2) = 7</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>It is optimal to not perform any operations.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>6</sup> &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Use a segment tree data structure to solve the problem.
2. Each node of the segment tree should store the subarray sum, the maximum subarray sum, the maximum prefix sum, and the maximum suffix sum within the subarray defined by that node.

## Solution

```rust
struct BlackNode {
    black_prefix: i64,
    black_suffix: i64,
    black_sum: i64,
    black_max_sum: i64,
}

impl BlackNode {
    fn black_new(val: i64) -> Self {
        Self {
            black_prefix: val,
            black_suffix: val,
            black_sum: val,
            black_max_sum: val,
        }
    }

    fn black_empty() -> Self {
        Self {
            black_prefix: 0,
            black_suffix: 0,
            black_sum: 0,
            black_max_sum: 0,
        }
    }

    fn black_change(&mut self, val: i64) {
        self.black_prefix = val;
        self.black_suffix = val;
        self.black_sum = val;
        self.black_max_sum = val;
    }
}

struct BlackSegTree {
    black_size: usize,
    black_tree: Vec<BlackNode>,
}

impl BlackSegTree {
    fn black_merge(a: &BlackNode, b: &BlackNode) -> BlackNode {
        BlackNode {
            black_sum: a.black_sum + b.black_sum,
            black_max_sum: a.black_max_sum.max(b.black_max_sum).max(a.black_suffix + b.black_prefix),
            black_prefix: a.black_prefix.max(a.black_sum + b.black_prefix),
            black_suffix: b.black_suffix.max(b.black_sum + a.black_suffix),
        }
    }

    fn black_build(&mut self, lx: usize, rx: usize, ni: usize, nums: &Vec<i32>) {
        if rx - lx == 1 {
            if lx < nums.len() {
                self.black_tree[ni] = BlackNode::black_new(nums[lx] as i64);
            }
            return;
        }
        let m = (lx + rx) / 2;
        self.black_build(lx, m, ni * 2 + 1, nums);
        self.black_build(m, rx, ni * 2 + 2, nums);
        self.black_tree[ni] = Self::black_merge(&self.black_tree[ni * 2 + 1], &self.black_tree[ni * 2 + 2]);
    }

    fn black_update(&mut self, idx: usize, val: i64, lx: usize, rx: usize, ni: usize) {
        if rx - lx == 1 {
            self.black_tree[ni].black_change(val);
            return;
        }
        let m = (lx + rx) / 2;
        if idx < m {
            self.black_update(idx, val, lx, m, ni * 2 + 1);
        } else {
            self.black_update(idx, val, m, rx, ni * 2 + 2);
        }
        self.black_tree[ni] = Self::black_merge(&self.black_tree[ni * 2 + 1], &self.black_tree[ni * 2 + 2]);
    }

    fn black_init(n: usize) -> Self {
        let mut black_size = 1;
        while black_size < n { black_size <<= 1; }
        Self {
            black_size,
            black_tree: (0..black_size * 2).map(|_| BlackNode::black_empty()).collect(),
        }
    }
}

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>) -> i64 {
        let black_n = nums.len();
        let mut black_max_num = i32::MIN;
        let mut black_indices: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();

        for i in 0..black_n {
            black_indices.entry(nums[i]).or_default().push(i);
            black_max_num = black_max_num.max(nums[i]);
        }

        if black_max_num <= 0 { return black_max_num as i64; }

        let mut black_st = BlackSegTree::black_init(black_n);
        black_st.black_build(0, black_st.black_size, 0, &nums);

        let mut black_ans = black_st.black_tree[0].black_max_sum;

        for (&black_val, black_idx_vec) in &black_indices {
            for &idx in black_idx_vec {
                black_st.black_update(idx, 0, 0, black_st.black_size, 0);
            }
            black_ans = black_ans.max(black_st.black_tree[0].black_max_sum);
            for &idx in black_idx_vec {
                black_st.black_update(idx, black_val as i64, 0, black_st.black_size, 0);
            }
        }

        black_ans
    }
}
```