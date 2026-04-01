impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut stack: Vec<i32> = vec![-1];
        let mut result = 0;

        for i in 0..n {
            if s[i] == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    result = result.max(i as i32 - stack.last().unwrap());
                }
            }
        }

        result
    }
}
