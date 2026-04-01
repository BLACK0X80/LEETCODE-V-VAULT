use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 { return n as i32; }
        let mut result = 2;

        for i in 0..n {
            let mut map: HashMap<(i32, i32), i32> = HashMap::new();
            for j in i + 1..n {
                let dy = points[j][1] - points[i][1];
                let dx = points[j][0] - points[i][0];
                let g = gcd(dy.abs(), dx.abs());
                let key = if dx == 0 { (1, 0) }
                          else if dy == 0 { (0, 1) }
                          else {
                              let (dy, dx) = (dy / g, dx / g);
                              if dx < 0 { (-dy, -dx) } else { (dy, dx) }
                          };
                let cnt = map.entry(key).or_insert(0);
                *cnt += 1;
                result = result.max(*cnt + 1);
            }
        }

        result
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
