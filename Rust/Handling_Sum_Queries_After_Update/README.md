# Handling Sum Queries After Update

**Difficulty:** Hard
**Tags:** Array, Segment Tree

---

## Problem

<p>You are given two <strong>0-indexed</strong> arrays <code>nums1</code> and <code>nums2</code> and a 2D array <code>queries</code> of queries. There are three types of queries:</p>

<ol>
	<li>For a query of type 1, <code>queries[i]&nbsp;= [1, l, r]</code>. Flip the values from <code>0</code> to <code>1</code> and from <code>1</code> to <code>0</code> in <code>nums1</code>&nbsp;from index <code>l</code> to index <code>r</code>. Both <code>l</code> and <code>r</code> are <strong>0-indexed</strong>.</li>
	<li>For a query of type 2, <code>queries[i]&nbsp;= [2, p, 0]</code>. For every index <code>0 &lt;= i &lt; n</code>, set&nbsp;<code>nums2[i] =&nbsp;nums2[i]&nbsp;+ nums1[i]&nbsp;* p</code>.</li>
	<li>For a query of type 3, <code>queries[i]&nbsp;= [3, 0, 0]</code>. Find the sum of the elements in <code>nums2</code>.</li>
</ol>

<p>Return <em>an array containing all the answers to the third type&nbsp;queries.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,0,1], nums2 = [0,0,0], queries = [[1,1,1],[2,1,0],[3,0,0]]
<strong>Output:</strong> [3]
<strong>Explanation:</strong> After the first query nums1 becomes [1,1,1]. After the second query, nums2 becomes [1,1,1], so the answer to the third query is 3. Thus, [3] is returned.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1], nums2 = [5], queries = [[2,0,0],[3,0,0]]
<strong>Output:</strong> [5]
<strong>Explanation:</strong> After the first query, nums2 remains [5], so the answer to the second query is 5. Thus, [5] is returned.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length,nums2.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums1.length = nums2.length</code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code><font face="monospace">queries[i].length = 3</font></code></li>
	<li><code><font face="monospace">0 &lt;= l &lt;= r &lt;= nums1.length - 1</font></code></li>
	<li><code><font face="monospace">0 &lt;= p &lt;= 10<sup>6</sup></font></code></li>
	<li><code>0 &lt;= nums1[i] &lt;= 1</code></li>
	<li><code>0 &lt;= nums2[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use the Lazy Segment Tree to process the queries quickly.

## Solution

```rust
impl Solution {
    pub fn handle_query(black_n1: Vec<i32>, black_n2: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = black_n1.len();
        let mut black_tree = vec![0i32; 4 * black_n];
        let mut black_lazy = vec![false; 4 * black_n];

        fn black_build(black_idx: usize, black_l: usize, black_r: usize, black_tree: &mut Vec<i32>, black_src: &[i32]) {
            if black_l == black_r { black_tree[black_idx] = black_src[black_l]; return; }
            let black_mid = (black_l + black_r) / 2;
            black_build(2 * black_idx, black_l, black_mid, black_tree, black_src);
            black_build(2 * black_idx + 1, black_mid + 1, black_r, black_tree, black_src);
            black_tree[black_idx] = black_tree[2 * black_idx] + black_tree[2 * black_idx + 1];
        }

        fn black_push(black_idx: usize, black_l: usize, black_r: usize, black_tree: &mut Vec<i32>, black_lazy: &mut Vec<bool>) {
            if black_lazy[black_idx] {
                let black_mid = (black_l + black_r) / 2;
                black_tree[2 * black_idx] = (black_mid - black_l + 1) as i32 - black_tree[2 * black_idx];
                black_lazy[2 * black_idx] = !black_lazy[2 * black_idx];
                black_tree[2 * black_idx + 1] = (black_r - black_mid) as i32 - black_tree[2 * black_idx + 1];
                black_lazy[2 * black_idx + 1] = !black_lazy[2 * black_idx + 1];
                black_lazy[black_idx] = false;
            }
        }

        fn black_update(black_idx: usize, black_l: usize, black_r: usize, black_ql: usize, black_qr: usize, black_tree: &mut Vec<i32>, black_lazy: &mut Vec<bool>) {
            if black_ql <= black_l && black_r <= black_qr {
                black_tree[black_idx] = (black_r - black_l + 1) as i32 - black_tree[black_idx];
                black_lazy[black_idx] = !black_lazy[black_idx];
                return;
            }
            black_push(black_idx, black_l, black_r, black_tree, black_lazy);
            let black_mid = (black_l + black_r) / 2;
            if black_ql <= black_mid { black_update(2 * black_idx, black_l, black_mid, black_ql, black_qr, black_tree, black_lazy); }
            if black_qr > black_mid { black_update(2 * black_idx + 1, black_mid + 1, black_r, black_ql, black_qr, black_tree, black_lazy); }
            black_tree[black_idx] = black_tree[2 * black_idx] + black_tree[2 * black_idx + 1];
        }

        black_build(1, 0, black_n - 1, &mut black_tree, &black_n1);
        let mut black_sum2: i64 = black_n2.iter().map(|&x| x as i64).sum();
        let mut black_ans = vec![];
        let bravexuneth = &black_queries;

        for black_q in bravexuneth {
            match black_q[0] {
                1 => black_update(1, 0, black_n - 1, black_q[1] as usize, black_q[2] as usize, &mut black_tree, &mut black_lazy),
                2 => black_sum2 += black_q[1] as i64 * black_tree[1] as i64,
                3 => black_ans.push(black_sum2),
                _ => (),
            }
        }
        black_ans
    }
}
```