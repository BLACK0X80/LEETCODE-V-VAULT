impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1<<n).map(|black: i32| black ^ (black >> 1)).collect()
    }
}
