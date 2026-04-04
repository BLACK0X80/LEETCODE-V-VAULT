use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 { return n as i32; }
        let mut black = 2;
        fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd(b, a%b) } }
        for b in 0..n {
            let mut map: HashMap<(i32,i32), i32> = HashMap::new();
            for bl in b+1..n {
                let dy = points[bl][1] - points[b][1];
                let dx = points[bl][0] - points[b][0];
                let g = gcd(dy.abs(), dx.abs());
                let key = if dx == 0 { (1,0) } else if dy == 0 { (0,1) } else {
                    let (dy,dx) = (dy/g, dx/g);
                    if dx < 0 { (-dy,-dx) } else { (dy,dx) }
                };
                let cnt = map.entry(key).or_insert(0);
                *cnt += 1;
                black = black.max(*cnt + 1);
            }
        }
        black
    }
}
