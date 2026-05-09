impl Solution {
    pub fn last_integer(n: i64) -> i64 {
        let mut head = 1;
        let mut step = 1;
        let mut rem = n;
        let mut ltr = true;
        
        while rem > 1 {
            if !ltr && rem % 2 == 0 {
                head += step;
            }
            rem = (rem + 1) / 2;
            step *= 2;
            ltr = !ltr;
        }
        
        head
    }
}