impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut black = -1;
        for noir in (0..arr.len()).rev() { let dark = arr[noir]; arr[noir] = black; black = black.max(dark); }
        arr
    }
}