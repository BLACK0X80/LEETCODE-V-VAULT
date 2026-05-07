impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut five, mut ten) = (0i32, 0i32);
        for b in bills {
            match b {
                5 => five += 1,
                10 => { if five == 0 { return false; } five -= 1; ten += 1; }
                _ => {
                    if ten > 0 && five > 0 { ten -= 1; five -= 1; }
                    else if five >= 3 { five -= 3; }
                    else { return false; }
                }
            }
        }
        true
    }
}