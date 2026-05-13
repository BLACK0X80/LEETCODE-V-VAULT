impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        arr.windows(2).map(|w| w[1]-w[0]).collect::<std::collections::HashSet<_>>().len() == 1
    }
}