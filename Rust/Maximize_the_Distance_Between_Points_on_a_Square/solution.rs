impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let s = side as i64;
        let k = k as usize;
        let mut black_pts: Vec<i64> = points.iter().map(|p| {
            let (x, y) = (p[0] as i64, p[1] as i64);
            if y == 0 { x }
            else if x == s { s + y }
            else if y == s { 2*s + (s-x) }
            else { 3*s + (s-y) }
        }).collect();
        black_pts.sort_unstable();
        let n = black_pts.len();
        let perim = 4*s;

        let black_check = |mid: i64| -> bool {
            for start in 0..n {
                let mut cur = black_pts[start];
                let mut cnt = 1;
                let mut ok = true;
                while cnt < k {
                    let need = cur + mid;
                    let idx = black_pts.partition_point(|&x| x < need);
                    if idx < n {
                        cur = black_pts[idx];
                        cnt += 1;
                    } else {
                        ok = false; break;
                    }
                }
                if ok {
                    let wrap = black_pts[start] + perim - cur;
                    if wrap >= mid { return true; }
                }
            }
            false
        };

        let (mut lo, mut hi) = (0i64, perim / k as i64);
        while lo < hi {
            let mid = (lo+hi+1)/2;
            if black_check(mid) { lo = mid; } else { hi = mid-1; }
        }
        lo as i32
    }
}