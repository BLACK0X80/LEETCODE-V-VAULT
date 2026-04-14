impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut black = 0.0f64;
        for b in 0..n { for bl in b+1..n { for blk in bl+1..n {
            let (x1,y1) = (points[b][0] as f64, points[b][1] as f64);
            let (x2,y2) = (points[bl][0] as f64, points[bl][1] as f64);
            let (x3,y3) = (points[blk][0] as f64, points[blk][1] as f64);
            black = black.max(((x1*(y2-y3)+x2*(y3-y1)+x3*(y1-y2))/2.0).abs());
        }}}
        black
    }
}