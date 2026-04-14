impl Solution {
    pub fn falling_squares(black1: Vec<Vec<i32>>) -> Vec<i32> {
        let mut black2 = vec![];
        let mut black3 = vec![];
        let mut black4 = 0;

        for p in black1 {
            let (l, side) = (p[0], p[1]);
            let r = l + side;
            let mut h = 0;
            for &(prev_l, prev_r, prev_h) in &black2 {
                if l < prev_r && r > prev_l {
                    h = h.max(prev_h);
                }
            }
            let curr_h = h + side;
            black2.push((l, r, curr_h));
            black4 = black4.max(curr_h);
            black3.push(black4);
        }
        black3
    }
}