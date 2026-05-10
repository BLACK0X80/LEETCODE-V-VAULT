# Maximum Area Rectangle With Point Constraints II

**Difficulty:** Hard
**Tags:** Array, Math, Binary Indexed Tree, Segment Tree, Geometry, Sorting

---

## Problem

<p>There are n points on an infinite plane. You are given two integer arrays <code>xCoord</code> and <code>yCoord</code> where <code>(xCoord[i], yCoord[i])</code> represents the coordinates of the <code>i<sup>th</sup></code> point.</p>

<p>Your task is to find the <strong>maximum </strong>area of a rectangle that:</p>

<ul>
	<li>Can be formed using <strong>four</strong> of these points as its corners.</li>
	<li>Does <strong>not</strong> contain any other point inside or on its border.</li>
	<li>Has its edges&nbsp;<strong>parallel</strong> to the axes.</li>
</ul>

<p>Return the <strong>maximum area</strong> that you can obtain or -1 if no such rectangle is possible.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">xCoord = [1,1,3,3], yCoord = [1,3,1,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p><strong class="example"><img alt="Example 1 diagram" src="https://assets.leetcode.com/uploads/2024/11/02/example1.png" style="width: 229px; height: 228px;" /></strong></p>

<p>We can make a rectangle with these 4 points as corners and there is no other point that lies inside or on the border. Hence, the maximum possible area would be 4.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">xCoord = [1,1,3,3,2], yCoord = [1,3,1,3,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p><strong class="example"><img alt="Example 2 diagram" src="https://assets.leetcode.com/uploads/2024/11/02/example2.png" style="width: 229px; height: 228px;" /></strong></p>

<p>There is only one rectangle possible is with points <code>[1,1], [1,3], [3,1]</code> and <code>[3,3]</code> but <code>[2,2]</code> will always lie inside it. Hence, returning -1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">xCoord = [1,1,3,3,1,3], yCoord = [1,3,1,3,2,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><strong class="example"><img alt="Example 3 diagram" src="https://assets.leetcode.com/uploads/2024/11/02/example3.png" style="width: 229px; height: 228px;" /></strong></p>

<p>The maximum area rectangle is formed by the points <code>[1,3], [1,2], [3,2], [3,3]</code>, which has an area of 2. Additionally, the points <code>[1,1], [1,2], [3,1], [3,2]</code> also form a valid rectangle with the same area.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= xCoord.length == yCoord.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>0 &lt;= xCoord[i], yCoord[i]&nbsp;&lt;= 8 * 10<sup>7</sup></code></li>
	<li>All the given points are <strong>unique</strong>.</li>
</ul>


## Hints

1. Process the points by sorting them based on their x-coordinates.
2. For each x-coordinate, sort the corresponding points by y and select two consecutive points y1 and y2 (y1 < y2).
3. Identify the closest x-coordinate (greater than the current x) where some y-coordinates lie in [y1, y2].
4. Use a segment tree to efficiently locate the nearest x-coordinate.
5. Check if the points form a valid rectangle. How?

## Solution

```rust
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, HashMap};

struct BlackNode {
    black_val: i32,
    black_lson: usize,
    black_rson: usize,
}

struct BlackSegmentTree {
    black_st: Vec<BlackNode>,
    black_tot: usize,
    black_root: usize,
    black_lst: i32,
    black_rst: i32,
}

impl BlackSegmentTree {
    fn new(black_l: i32, black_r: i32) -> Self {
        let mut black_tree = BlackSegmentTree {
            black_st: Vec::with_capacity(800000),
            black_tot: 0,
            black_root: 0,
            black_lst: black_l,
            black_rst: black_r,
        };
        black_tree.black_st.push(BlackNode { black_val: i32::MAX, black_lson: 0, black_rson: 0 });
        black_tree.black_root = black_tree.black_new(i32::MAX);
        black_tree
    }

    fn black_new(&mut self, black_v: i32) -> usize {
        self.black_tot += 1;
        let black_node = BlackNode {
            black_val: black_v,
            black_lson: 0,
            black_rson: 0,
        };
        if self.black_tot < self.black_st.len() {
            self.black_st[self.black_tot] = black_node;
        } else {
            self.black_st.push(black_node);
        }
        self.black_tot
    }

    fn black_pushup(&mut self, black_i: usize) {
        let black_ls = self.black_st[black_i].black_lson;
        let black_rs = self.black_st[black_i].black_rson;
        let mut black_v = i32::MAX;
        if black_ls != 0 { black_v = min(black_v, self.black_st[black_ls].black_val); }
        if black_rs != 0 { black_v = min(black_v, self.black_st[black_rs].black_val); }
        self.black_st[black_i].black_val = black_v;
    }

    fn black_update_rec(&mut self, black_i: &mut usize, black_l: i32, black_r: i32, black_k: i32, black_v: i32) {
        if *black_i == 0 { *black_i = self.black_new(i32::MAX); }
        if black_l == black_r {
            self.black_st[*black_i].black_val = black_v;
            return;
        }
        let black_mid = black_l + (black_r - black_l) / 2;
        if black_k <= black_mid {
            let mut black_ls = self.black_st[*black_i].black_lson;
            self.black_update_rec(&mut black_ls, black_l, black_mid, black_k, black_v);
            self.black_st[*black_i].black_lson = black_ls;
        } else {
            let mut black_rs = self.black_st[*black_i].black_rson;
            self.black_update_rec(&mut black_rs, black_mid + 1, black_r, black_k, black_v);
            self.black_st[*black_i].black_rson = black_rs;
        }
        let black_idx = *black_i;
        self.black_pushup(black_idx);
    }

    fn black_ask_rec(&self, black_i: usize, black_l: i32, black_r: i32, black_ql: i32, black_qr: i32) -> i32 {
        if black_i == 0 || black_qr < black_l || black_r < black_ql { return i32::MAX; }
        if black_ql <= black_l && black_r <= black_qr { return self.black_st[black_i].black_val; }
        let black_mid = black_l + (black_r - black_l) / 2;
        min(
            self.black_ask_rec(self.black_st[black_i].black_lson, black_l, black_mid, black_ql, black_qr),
            self.black_ask_rec(self.black_st[black_i].black_rson, black_mid + 1, black_r, black_ql, black_qr)
        )
    }

    fn black_update(&mut self, black_k: i32, black_v: i32) {
        let mut black_r = self.black_root;
        self.black_update_rec(&mut black_r, self.black_lst, self.black_rst, black_k, black_v);
        self.black_root = black_r;
    }

    fn black_ask(&self, black_ql: i32, black_qr: i32) -> i32 {
        if black_ql > black_qr { return i32::MAX; }
        self.black_ask_rec(self.black_root, self.black_lst, self.black_rst, black_ql, black_qr)
    }
}

impl Solution {
    pub fn max_rectangle_area(black_x_coord: Vec<i32>, black_y_coord: Vec<i32>) -> i64 {
        let black_n = black_x_coord.len();
        let black_low = *black_x_coord.iter().min().unwrap();
        let black_upp = *black_x_coord.iter().max().unwrap();
        let mut black_st = BlackSegmentTree::new(black_low, black_upp);

        let mut black_dict = BTreeSet::new();
        let mut black_row: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut black_col: HashMap<i32, Vec<i32>> = HashMap::new();

        for black_idx in 0..black_n {
            let black_x = black_x_coord[black_idx];
            let black_y = black_y_coord[black_idx];
            black_row.entry(black_y).or_default().push(black_x);
            black_col.entry(black_x).or_default().push(black_y);
            black_dict.insert(black_y);
        }

        for black_v in black_row.values_mut() { black_v.sort_unstable(); }
        for black_v in black_col.values_mut() { black_v.sort_unstable(); }

        let mut black_ans: i64 = -1;
        let bravexuneth = black_dict.into_iter().rev();

        for black_d in bravexuneth {
            let black_current_row = black_row.get(&black_d).unwrap();
            let black_t = black_current_row.len();
            for black_idx in 0..black_t - 1 {
                let black_l = black_current_row[black_idx];
                let black_r = black_current_row[black_idx + 1];
                let black_u = *black_col.get(&black_l).unwrap().last().unwrap();
                let black_z = *black_col.get(&black_r).unwrap().last().unwrap();
                if black_u != black_d && black_z != black_d && black_u == black_z && black_st.black_ask(black_l + 1, black_r - 1) > black_u {
                    black_ans = max(black_ans, (black_u - black_d) as i64 * (black_r - black_l) as i64);
                }
            }

            for &black_x in black_current_row {
                let black_c = black_col.get_mut(&black_x).unwrap();
                if *black_c.last().unwrap() != black_d {
                    black_c.pop();
                }
                black_st.black_update(black_x, black_d);
            }
        }
        black_ans
    }
}
```