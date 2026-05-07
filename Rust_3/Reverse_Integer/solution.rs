impl Solution {
    pub fn reverse(black: i32) -> i32 {
        black.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap_or(0) * black.signum()
    }
}