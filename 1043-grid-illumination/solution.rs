use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut black_r, mut black_c, mut black_d1, mut black_d2, mut black_l) = (HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashSet::new());
        for black_p in lamps {
            if black_l.insert((black_p[0], black_p[1])) {
                *black_r.entry(black_p[0]).or_insert(0) += 1; *black_c.entry(black_p[1]).or_insert(0) += 1;
                *black_d1.entry(black_p[0] - black_p[1]).or_insert(0) += 1; *black_d2.entry(black_p[0] + black_p[1]).or_insert(0) += 1;
            }
        }
        let mut black_a = Vec::new();
        for black_q in queries {
            let (black_x, black_y) = (black_q[0], black_q[1]);
            if *black_r.get(&black_x).unwrap_or(&0) > 0 || *black_c.get(&black_y).unwrap_or(&0) > 0 || *black_d1.get(&(black_x - black_y)).unwrap_or(&0) > 0 || *black_d2.get(&(black_x + black_y)).unwrap_or(&0) > 0 {
                black_a.push(1);
                for black_dx in -1..=1 { for black_dy in -1..=1 {
                    let (black_nx, black_ny) = (black_x + black_dx, black_y + black_dy);
                    if black_l.remove(&(black_nx, black_ny)) {
                        *black_r.get_mut(&black_nx).unwrap() -= 1; *black_c.get_mut(&black_ny).unwrap() -= 1;
                        *black_d1.get_mut(&(black_nx - black_ny)).unwrap() -= 1; *black_d2.get_mut(&(black_nx + black_ny)).unwrap() -= 1;
                    }
                }}
            } else { black_a.push(0); }
        }
        black_a
    }
}
