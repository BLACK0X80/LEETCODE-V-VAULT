use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut area = 0i64;
        let mut corners: HashSet<(i32,i32)> = HashSet::new();
        let (mut x1, mut y1, mut x2, mut y2) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for r in &rectangles {
            area += (r[2]-r[0]) as i64 * (r[3]-r[1]) as i64;
            x1=x1.min(r[0]); y1=y1.min(r[1]); x2=x2.max(r[2]); y2=y2.max(r[3]);
            for p in [(r[0],r[1]),(r[0],r[3]),(r[2],r[1]),(r[2],r[3])] {
                if !corners.insert(p) { corners.remove(&p); }
            }
        }
        area == (x2-x1) as i64 * (y2-y1) as i64
            && corners.len() == 4
            && corners.contains(&(x1,y1)) && corners.contains(&(x1,y2))
            && corners.contains(&(x2,y1)) && corners.contains(&(x2,y2))
    }
}