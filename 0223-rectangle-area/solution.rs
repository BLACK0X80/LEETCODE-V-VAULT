impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let black = (ax2-ax1)*(ay2-ay1) + (bx2-bx1)*(by2-by1);
        let ox = ax1.max(bx1); let oy = ay1.max(by1);
        let ox2 = ax2.min(bx2); let oy2 = ay2.min(by2);
        let overlap = if ox < ox2 && oy < oy2 { (ox2-ox)*(oy2-oy) } else { 0 };
        black - overlap
    }
}
