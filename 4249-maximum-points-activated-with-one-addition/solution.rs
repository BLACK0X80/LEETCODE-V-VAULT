use std::collections::HashMap;

impl Solution {
    pub fn max_activated(black_pts: Vec<Vec<i32>>) -> i32 {
        let black_n = black_pts.len();
        let mut black_p: Vec<usize> = (0..black_n).collect();
        let mut black_sz = vec![1; black_n];

        fn black_find(i: usize, p: &mut Vec<usize>) -> usize {
            if p[i] == i { i }
            else { p[i] = black_find(p[i], p); p[i] }
        }

        let mut black_unite = |i: usize, j: usize, p: &mut Vec<usize>, s: &mut Vec<i32>| {
            let (r1, r2) = (black_find(i, p), black_find(j, p));
            if r1 != r2 {
                let (black_hi, black_lo) = if s[r1] >= s[r2] { (r1, r2) } else { (r2, r1) };
                p[black_lo] = black_hi;
                s[black_hi] += s[black_lo];
            }
        };

        let (mut black_xm, mut black_ym) = (HashMap::new(), HashMap::new());
        for i in 0..black_n {
            let (x, y) = (black_pts[i][0], black_pts[i][1]);
            if let Some(&prev) = black_xm.get(&x) { black_unite(i, prev, &mut black_p, &mut black_sz); }
            else { black_xm.insert(x, i); }
            if let Some(&prev) = black_ym.get(&y) { black_unite(i, prev, &mut black_p, &mut black_sz); }
            else { black_ym.insert(y, i); }
        }

        let (mut black_m1, mut black_m2) = (0, 0);
        for i in 0..black_n {
            if black_p[i] == i {
                let s = black_sz[i];
                if s > black_m1 { black_m2 = black_m1; black_m1 = s; }
                else if s > black_m2 { black_m2 = s; }
            }
        }
        black_m1 + black_m2 + 1
    }
}
