impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut events = Vec::new();
        for r in &rectangles {
            let (x1, y1, x2, y2) = (r[0], r[1], r[2], r[3]);
            events.push((y1, 1, x1, x2));
            events.push((y2, -1, x1, x2));
        }
        events.sort_by_key(|e| (e.0, -e.1));
        let mut active: Vec<(i32, i32)> = Vec::new();
        let mut area = 0i64;
        let mut prev_y = events[0].0;
        for (y, typ, x1, x2) in events {
            if y != prev_y && !active.is_empty() {
                let height = (y - prev_y) as i64;
                active.sort();
                let mut width = 0i64;
                let mut cur_l = -1;
                let mut cur_r = -1;
                for &(l, r) in &active {
                    if l > cur_r {
                        if cur_r != -1 { width += (cur_r - cur_l) as i64; }
                        cur_l = l; cur_r = r;
                    } else if r > cur_r {
                        cur_r = r;
                    }
                }
                if cur_r != -1 { width += (cur_r - cur_l) as i64; }
                area = (area + width * height) % MOD;
            }
            if typ == 1 {
                active.push((x1, x2));
            } else {
                if let Some(pos) = active.iter().position(|&x| x == (x1, x2)) {
                    active.remove(pos);
                }
            }
            prev_y = y;
        }
        area as i32
    }
}