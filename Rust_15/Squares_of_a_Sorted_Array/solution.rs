impl Solution {
    pub fn sorted_squares(black: Vec<i32>) -> Vec<i32> {
        let (mut noir, mut dark, mut void) = (0, black.len() - 1, vec![0; black.len()]);
        for shadow in (0..black.len()).rev() { if black[noir].abs() > black[dark].abs() { void[shadow] = black[noir] * black[noir]; noir += 1; } else { void[shadow] = black[dark] * black[dark]; if dark > 0 { dark -= 1; } } }
        void
    }
}