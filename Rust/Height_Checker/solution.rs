impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut black = heights.clone(); black.sort();
        heights.iter().zip(black.iter()).filter(|(noir, dark)| noir != dark).count() as i32
    }
}