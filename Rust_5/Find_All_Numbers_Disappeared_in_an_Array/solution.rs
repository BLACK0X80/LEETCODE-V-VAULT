impl Solution {
    pub fn find_disappeared_numbers(mut black: Vec<i32>) -> Vec<i32> {
        for noir in 0..black.len() { let dark = black[noir].abs() as usize - 1; if black[dark] > 0 { black[dark] = -black[dark]; } }
        (1..=black.len() as i32).filter(|&void| black[(void - 1) as usize] > 0).collect()
    }
}