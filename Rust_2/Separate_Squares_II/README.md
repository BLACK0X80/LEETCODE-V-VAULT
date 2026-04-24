# Separate Squares II

**Difficulty:** Hard
**Tags:** Array, Binary Search, Segment Tree, Sweep Line

---

## Problem

<p>You are given a 2D integer array <code>squares</code>. Each <code>squares[i] = [x<sub>i</sub>, y<sub>i</sub>, l<sub>i</sub>]</code> represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.</p>

<p>Find the <strong>minimum</strong> y-coordinate value of a horizontal line such that the total area covered by squares above the line <em>equals</em> the total area covered by squares below the line.</p>

<p>Answers within <code>10<sup>-5</sup></code> of the actual answer will be accepted.</p>

<p><strong>Note</strong>: Squares <strong>may</strong> overlap. Overlapping areas should be counted <strong>only once</strong> in this version.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">squares = [[0,0,1],[2,2,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1.00000</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/01/15/4065example1drawio.png" style="width: 269px; height: 203px;" /></p>

<p>Any horizontal line between <code>y = 1</code> and <code>y = 2</code> results in an equal split, with 1 square unit above and 1 square unit below. The minimum y-value is 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">squares = [[0,0,2],[1,1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1.00000</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/01/15/4065example2drawio.png" style="width: 269px; height: 203px;" /></p>

<p>Since the blue square overlaps with the red square, it will not be counted again. Thus, the line <code>y = 1</code> splits the squares into two equal parts.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= squares.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>squares[i] = [x<sub>i</sub>, y<sub>i</sub>, l<sub>i</sub>]</code></li>
	<li><code>squares[i].length == 3</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= l<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li>The total area of all the squares will not exceed <code>10<sup>15</sup></code>.</li>
</ul>


## Hints

1. Use a line sweep and a segment tree.
2. The line must lie in one of the squares.

## Solution

```rust
struct Seg {
    cnt: Vec<i32>,
    cov: Vec<i64>,
    xs: Vec<i64>,
    n: usize,
}

impl Seg {
    fn new(xs: Vec<i64>) -> Self {
        let n = xs.len() - 1;
        Seg { cnt: vec![0; 4*n], cov: vec![0; 4*n], xs, n }
    }

    fn push(&mut self, ql: i64, qr: i64, v: i32, l: usize, r: usize, p: usize) {
        if self.xs[r+1] <= ql || self.xs[l] >= qr { return; }
        if ql <= self.xs[l] && self.xs[r+1] <= qr { self.cnt[p] += v; }
        else {
            let m = (l+r)/2;
            self.push(ql, qr, v, l, m, 2*p+1);
            self.push(ql, qr, v, m+1, r, 2*p+2);
        }
        self.cov[p] = if self.cnt[p] > 0 { self.xs[r+1] - self.xs[l] }
                      else if l == r { 0 }
                      else { self.cov[2*p+1] + self.cov[2*p+2] };
    }

    fn upd(&mut self, ql: i64, qr: i64, v: i32) { let n = self.n; self.push(ql, qr, v, 0, n-1, 0); }
    fn get(&self) -> i64 { self.cov[0] }
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut black: Vec<(i64, i32, i64, i64)> = vec![];
        let mut black_xs: std::collections::BTreeSet<i64> = std::collections::BTreeSet::new();

        for sq in &squares {
            let (x, y, s) = (sq[0] as i64, sq[1] as i64, sq[2] as i64);
            black.push((y, 1, x, x+s));
            black.push((y+s, -1, x, x+s));
            black_xs.insert(x); black_xs.insert(x+s);
        }

        black.sort_unstable_by_key(|b| b.0);
        let xs: Vec<i64> = black_xs.into_iter().collect();
        let mut seg = Seg::new(xs);

        let mut black_area = 0i64;
        let mut black_prev = black[0].0;
        let mut black_widths: Vec<i64> = vec![];

        for &(y, d, xl, xr) in &black {
            black_area += seg.get() * (y - black_prev);
            seg.upd(xl, xr, d);
            black_widths.push(seg.get());
            black_prev = y;
        }

        let black_half = black_area as f64 / 2.0;
        let mut black_acc = 0.0f64;

        for i in 0..black.len()-1 {
            let w = black_widths[i] as f64;
            let h = (black[i+1].0 - black[i].0) as f64;
            let a = w * h;
            if black_acc + a >= black_half {
                return black[i].0 as f64 + (black_half - black_acc) / w;
            }
            black_acc += a;
        }

        black[black.len()-1].0 as f64
    }
}
```