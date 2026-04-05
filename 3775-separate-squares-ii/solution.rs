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
