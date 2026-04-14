impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right).filter(|&black| {
            let mut b = black;
            while b > 0 {
                let d = b % 10;
                if d == 0 || black % d != 0 { return false; }
                b /= 10;
            }
            true
        }).collect()
    }
}