impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut count = 0;
        let mut black = String::new();
        Self::backtrack(n, k, &mut count, &mut black, &mut String::new())
    }
    fn backtrack(n: i32, k: i32, count: &mut i32, black: &mut String, result: &mut String) -> String {
        if !result.is_empty() { return result.clone(); }
        if black.len() as i32 == n {
            *count += 1;
            if *count == k { return black.clone(); }
            return String::new();
        }
        for c in ['a', 'b', 'c'] {
            if black.chars().last().map_or(true, |last| last != c) {
                black.push(c);
                let found = Self::backtrack(n, k, count, black, result);
                if !found.is_empty() { return found; }
                black.pop();
            }
        }
        String::new()
    }
}