impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut black = Vec::new();
        Self::backtrack(1, n, k, &mut black, &mut result);
        result
    }
    fn backtrack(start: i32, n: i32, k: i32, black: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if black.len() as i32 == k {
            result.push(black.clone());
            return;
        }
        for i in start..=n {
            black.push(i);
            Self::backtrack(i + 1, n, k, black, result);
            black.pop();
        }
    }
}