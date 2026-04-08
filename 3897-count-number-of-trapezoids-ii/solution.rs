use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_trapezoids(black_pts: Vec<Vec<i32>>) -> i64 {
        let black_n = black_pts.len();
        let mut black_mid_map: HashMap<i64, (i64, HashMap<i64, i32>)> = HashMap::new();
        let mut black_slope_map: HashMap<i64, HashMap<i64, HashSet<usize>>> = HashMap::new();

        fn black_gcd(a: i32, b: i32) -> i32 { if b == 0 { a.abs() } else { black_gcd(b, a % b) } }

        for i in 0..black_n {
            for j in i + 1..black_n {
                let (x1, y1) = (black_pts[i][0], black_pts[i][1]);
                let (x2, y2) = (black_pts[j][0], black_pts[j][1]);

                let (mut dx, mut dy) = (x2 - x1, y2 - y1);
                let g = black_gcd(dx, dy);
                dx /= g; dy /= g;
                if dx < 0 || (dx == 0 && dy < 0) { dx = -dx; dy = -dy; }

                let black_s_key = ((dy as i64 + 2000) << 32) | (dx as i64 + 2000);
                let black_l_key = dy as i64 * x1 as i64 - dx as i64 * y1 as i64;
                black_slope_map.entry(black_s_key).or_default().entry(black_l_key).or_default().extend([i, j]);

                let black_m_key = (((x1 + x2) as i64 + 4000) << 32) | ((y1 + y2) as i64 + 4000);
                let black_mid_info = black_mid_map.entry(black_m_key).or_insert((0, HashMap::new()));
                black_mid_info.0 += 1;
                *black_mid_info.1.entry(black_s_key).or_default() += 1;
            }
        }

        let mut black_para = 0i64;
        for (_, (black_tot, black_dirs)) in black_mid_map {
            if black_tot < 2 { continue; }
            let mut black_bad = 0i64;
            for &c in black_dirs.values() { black_bad += c as i64 * (c as i64 - 1) / 2; }
            black_para += black_tot * (black_tot - 1) / 2 - black_bad;
        }

        let mut black_total_para = 0i64;
        for black_line_map in black_slope_map.values() {
            let black_a: Vec<i64> = black_line_map.values()
                .filter(|v| v.len() >= 2)
                .map(|v| { let c = v.len() as i64; c * (c - 1) / 2 })
                .collect();
            if black_a.len() < 2 { continue; }
            let (mut s, mut s2) = (0i64, 0i64);
            for x in black_a { s += x; s2 += x * x; }
            black_total_para += (s * s - s2) / 2;
        }

        black_total_para - black_para
    }
}
