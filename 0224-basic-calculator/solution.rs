impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<(i32, i32)> = vec![];
        let mut result = 0;
        let mut sign = 1;
        let mut i = 0;
        let s = s.as_bytes();

        while i < s.len() {
            match s[i] {
                b' ' => {}
                b'+' => sign = 1,
                b'-' => sign = -1,
                b'(' => {
                    stack.push((result, sign));
                    result = 0;
                    sign = 1;
                }
                b')' => {
                    let (prev_result, prev_sign) = stack.pop().unwrap();
                    result = prev_result + prev_sign * result;
                }
                _ => {
                    let mut num = 0i32;
                    while i < s.len() && s[i].is_ascii_digit() {
                        num = num * 10 + (s[i] - b'0') as i32;
                        i += 1;
                    }
                    result += sign * num;
                    continue;
                }
            }
            i += 1;
        }

        result
    }
}
