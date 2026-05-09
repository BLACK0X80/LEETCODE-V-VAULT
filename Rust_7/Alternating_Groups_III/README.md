# Alternating Groups III

**Difficulty:** Hard
**Tags:** Array, Binary Indexed Tree, Ordered Set

---

## Problem

<p>There are some red and blue tiles arranged circularly. You are given an array of integers <code>colors</code> and a 2D integers array <code>queries</code>.</p>

<p>The color of tile <code>i</code> is represented by <code>colors[i]</code>:</p>

<ul>
	<li><code>colors[i] == 0</code> means that tile <code>i</code> is <strong>red</strong>.</li>
	<li><code>colors[i] == 1</code> means that tile <code>i</code> is <strong>blue</strong>.</li>
</ul>

<p>An <strong>alternating</strong> group is a contiguous subset of tiles in the circle with <strong>alternating</strong> colors (each tile in the group except the first and last one has a different color from its <b>adjacent</b> tiles in the group).</p>

<p>You have to process queries of two types:</p>

<ul>
	<li><code>queries[i] = [1, size<sub>i</sub>]</code>, determine the count of <strong>alternating</strong> groups with size <code>size<sub>i</sub></code>.</li>
	<li><code>queries[i] = [2, index<sub>i</sub>, color<sub>i</sub>]</code>, change <code>colors[index<sub>i</sub>]</code> to <code>color<font face="monospace"><sub>i</sub></font></code>.</li>
</ul>

<p>Return an array <code>answer</code> containing the results of the queries of the first type <em>in order</em>.</p>

<p><strong>Note</strong> that since <code>colors</code> represents a <strong>circle</strong>, the <strong>first</strong> and the <strong>last</strong> tiles are considered to be next to each other.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">colors = [0,1,1,0,1], queries = [[2,1,0],[1,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2]</span></p>

<p><strong>Explanation:</strong></p>

<p><strong class="example"><img alt="" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage="" src="https://assets.leetcode.com/uploads/2024/06/03/screenshot-from-2024-06-03-20-14-44.png" style="width: 150px; height: 150px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; --darkreader-inline-bgimage: initial; --darkreader-inline-bgcolor: #181a1b;" /></strong></p>

<p>First query:</p>

<p>Change <code>colors[1]</code> to 0.</p>

<p><img alt="" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage="" src="https://assets.leetcode.com/uploads/2024/06/03/screenshot-from-2024-06-03-20-20-25.png" style="width: 150px; height: 150px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; --darkreader-inline-bgimage: initial; --darkreader-inline-bgcolor: #181a1b;" /></p>

<p>Second query:</p>

<p>Count of the alternating groups with size 4:</p>

<p><img alt="" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage="" src="https://assets.leetcode.com/uploads/2024/06/03/screenshot-from-2024-06-03-20-25-02-2.png" style="width: 150px; height: 150px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; --darkreader-inline-bgimage: initial; --darkreader-inline-bgcolor: #181a1b;" /><img alt="" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage="" src="https://assets.leetcode.com/uploads/2024/06/03/screenshot-from-2024-06-03-20-24-12.png" style="width: 150px; height: 150px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; --darkreader-inline-bgimage: initial; --darkreader-inline-bgcolor: #181a1b;" /></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">colors = [0,0,1,0,1,1], queries = [[1,3],[2,3,0],[1,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,0]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage="" src="https://assets.leetcode.com/uploads/2024/06/03/screenshot-from-2024-06-03-20-35-50.png" style="width: 150px; height: 150px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; --darkreader-inline-bgimage: initial; --darkreader-inline-bgcolor: #181a1b;" /></p>

<p>First query:</p>

<p>Count of the alternating groups with size 3:</p>

<p><img alt="" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage="" src="https://assets.leetcode.com/uploads/2024/06/03/screenshot-from-2024-06-03-20-37-13.png" style="width: 150px; height: 150px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; --darkreader-inline-bgimage: initial; --darkreader-inline-bgcolor: #181a1b;" /><img alt="" data-darkreader-inline-bgcolor="" data-darkreader-inline-bgimage="" src="https://assets.leetcode.com/uploads/2024/06/03/screenshot-from-2024-06-03-20-36-40.png" style="width: 150px; height: 150px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; --darkreader-inline-bgimage: initial; --darkreader-inline-bgcolor: #181a1b;" /></p>

<p>Second query: <code>colors</code> will not change.</p>

<p>Third query: There is no alternating group with size 5.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>4 &lt;= colors.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= colors[i] &lt;= 1</code></li>
	<li><code>1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>queries[i][0] == 1</code> or <code>queries[i][0] == 2</code></li>
	<li>For all <code>i</code> that:
	<ul>
		<li><code>queries[i][0] == 1</code>: <code>queries[i].length == 2</code>, <code>3 &lt;= queries[i][1] &lt;= colors.length - 1</code></li>
		<li><code>queries[i][0] == 2</code>: <code>queries[i].length == 3</code>, <code>0 &lt;= queries[i][1] &lt;= colors.length - 1</code>, <code>0 &lt;= queries[i][2] &lt;= 1</code></li>
	</ul>
	</li>
</ul>


## Hints

1. Try using a segment tree to store the maximal alternating groups.
2. Store the sizes of these maximal alternating groups in another data structure.
3. Find the count of the alternating groups of size <code>k</code> with having the count of maximal alternating groups with size greater than or equal to <code>k</code> and the sum of their sizes.

## Solution

```rust
use std::collections::BTreeSet;

struct BlackSegTree {
    black_n: usize,
    black_tree_val: Vec<i32>,
    black_tree_cnt: Vec<i32>,
}

impl BlackSegTree {
    fn new(black_n: usize) -> Self {
        Self {
            black_n,
            black_tree_val: vec![0; 4 * black_n],
            black_tree_cnt: vec![0; 4 * black_n],
        }
    }

    fn black_query_sum(&self, black_x: usize, black_y: usize, black_l: usize, black_r: usize, black_i: usize) -> i32 {
        if black_r < black_x || black_l > black_y { return 0; }
        if black_l >= black_x && black_r <= black_y { return self.black_tree_val[black_i]; }
        let black_m = (black_l + black_r) >> 1;
        self.black_query_sum(black_x, black_y, black_l, black_m, black_i * 2 + 1) +
        self.black_query_sum(black_x, black_y, black_m + 1, black_r, black_i * 2 + 2)
    }

    fn black_query_cnt(&self, black_x: usize, black_y: usize, black_l: usize, black_r: usize, black_i: usize) -> i32 {
        if black_r < black_x || black_l > black_y { return 0; }
        if black_l >= black_x && black_r <= black_y { return self.black_tree_cnt[black_i]; }
        let black_m = (black_l + black_r) >> 1;
        self.black_query_cnt(black_x, black_y, black_l, black_m, black_i * 2 + 1) +
        self.black_query_cnt(black_x, black_y, black_m + 1, black_r, black_i * 2 + 2)
    }

    fn black_update(&mut self, black_ind: usize, black_val: i32, black_l: usize, black_r: usize, black_i: usize) {
        if black_l == black_r {
            self.black_tree_cnt[black_i] += black_val;
            self.black_tree_val[black_i] = self.black_tree_cnt[black_i] * black_l as i32;
            return;
        }
        let black_m = (black_l + black_r) >> 1;
        if black_m >= black_ind { self.black_update(black_ind, black_val, black_l, black_m, black_i * 2 + 1); }
        else { self.black_update(black_ind, black_val, black_m + 1, black_r, black_i * 2 + 2); }
        self.black_tree_val[black_i] = self.black_tree_val[black_i * 2 + 1] + self.black_tree_val[black_i * 2 + 2];
        self.black_tree_cnt[black_i] = self.black_tree_cnt[black_i * 2 + 1] + self.black_tree_cnt[black_i * 2 + 2];
    }
}

impl Solution {
    pub fn number_of_alternating_groups(black_colors: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_colors.len();
        let mut black_arr = vec![0; 2 * black_n];
        for black_j in 0..black_n { black_arr[black_j] = black_colors[black_j]; black_arr[black_n + black_j] = black_colors[black_j]; }
        let mut black_st = BlackSegTree::new(2 * black_n);
        let mut black_all: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut black_j = 0;
        while black_j < 2 * black_n - 1 {
            let black_l = black_j;
            let mut black_r = black_j + 1;
            while black_r < 2 * black_n - 1 && black_arr[black_r] != black_arr[black_r - 1] { black_r += 1; }
            Self::black_insert((black_l, black_r - 1), &mut black_all, &mut black_st, black_n);
            black_j = black_r;
        }

        let mut black_result = Vec::new();
        for black_q in black_queries {
            if black_q[0] == 1 {
                black_result.push(Self::black_count(black_q[1], Self::black_group_at(black_n, &black_all), &black_st, black_n));
            } else {
                let (black_idx, black_val) = (black_q[1] as usize, black_q[2]);
                Self::black_update_group(black_idx, black_val, &mut black_all, &mut black_arr, &mut black_st, black_n);
                Self::black_update_group(black_idx + black_n, black_val, &mut black_all, &mut black_arr, &mut black_st, black_n);
            }
        }
        black_result
    }

    fn black_insert(black_val: (usize, usize), black_all: &mut BTreeSet<(usize, usize)>, black_st: &mut BlackSegTree, black_n: usize) {
        black_all.insert(black_val);
        if black_val.0 < black_n { black_st.black_update(black_val.1 - black_val.0 + 1, 1, 0, black_st.black_n - 1, 0); }
    }

    fn black_remove(black_val: (usize, usize), black_all: &mut BTreeSet<(usize, usize)>, black_st: &mut BlackSegTree, black_n: usize) {
        black_all.remove(&black_val);
        if black_val.0 < black_n { black_st.black_update(black_val.1 - black_val.0 + 1, -1, 0, black_st.black_n - 1, 0); }
    }

    fn black_group_at(black_ind: usize, black_all: &BTreeSet<(usize, usize)>) -> (usize, usize) {
        let black_res = black_all.range(..(black_ind + 1, 0)).next_back().or_else(|| black_all.range((black_ind, 0)..).next()).unwrap();
        *black_res
    }

    fn black_count(black_q: i32, black_last: (usize, usize), black_st: &BlackSegTree, black_n: usize) -> i32 {
        let black_sum = black_st.black_query_sum(black_q as usize, black_st.black_n - 1, 0, black_st.black_n - 1, 0);
        let black_cnt = black_st.black_query_cnt(black_q as usize, black_st.black_n - 1, 0, black_st.black_n - 1, 0);
        let mut black_ans = black_sum - (black_q - 1) * black_cnt;
        let (black_l, black_r) = black_last;
        if black_l >= black_n || (black_r - black_l + 1) < black_q as usize { return black_ans; }
        if black_r >= black_n {
            let black_can = black_n - black_l;
            let black_has = (black_r - black_l + 1) - (black_q as usize - 1);
            if black_can < black_has { black_ans -= (black_has - black_can) as i32; }
        }
        black_ans
    }

    fn black_update_group(black_ind: usize, black_val: i32, black_all: &mut BTreeSet<(usize, usize)>, black_arr: &mut Vec<i32>, black_st: &mut BlackSegTree, black_n: usize) {
        if black_ind == 2 * black_n - 1 || black_arr[black_ind] == black_val { return; }
        black_arr[black_ind] = black_val;
        let black_with_ind = Self::black_group_at(black_ind, black_all);
        Self::black_remove(black_with_ind, black_all, black_st, black_n);
        let (mut black_l, mut black_r) = black_with_ind;
        if black_l < black_ind && black_r > black_ind {
            Self::black_insert((black_l, black_ind - 1), black_all, black_st, black_n);
            Self::black_insert((black_ind, black_ind), black_all, black_st, black_n);
            Self::black_insert((black_ind + 1, black_r), black_all, black_st, black_n);
            return;
        }
        if black_l == black_ind && black_r != black_ind { Self::black_insert((black_l + 1, black_r), black_all, black_st, black_n); }
        if black_r == black_ind && black_l != black_ind { Self::black_insert((black_l, black_r - 1), black_all, black_st, black_n); }
        black_l = black_ind; black_r = black_ind;
        let mut black_to_remove = Vec::new();
        while black_l > 0 && black_arr[black_l - 1] != black_arr[black_l] {
            let black_prev = Self::black_group_at(black_l - 1, black_all);
            black_to_remove.push(black_prev);
            black_l = black_prev.0;
        }
        while black_r < 2 * black_n - 2 && black_arr[black_r + 1] != black_arr[black_r] {
            let black_next = Self::black_group_at(black_r + 1, black_all);
            black_to_remove.push(black_next);
            black_r = black_next.1;
        }
        for black_i in black_to_remove { Self::black_remove(black_i, black_all, black_st, black_n); }
        Self::black_insert((black_l, black_r), black_all, black_st, black_n);
    }
}
```