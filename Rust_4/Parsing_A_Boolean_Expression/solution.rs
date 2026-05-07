impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        for b in expression.bytes() {
            match b {
                b',' => {}
                b't' | b'f' | b'!' | b'&' | b'|' => stack.push(b),
                b'(' => stack.push(b'('),
                b')' => {
                    let mut has_t = false;
                    let mut has_f = false;
                    while *stack.last().unwrap() != b'(' {
                        match stack.pop().unwrap() {
                            b't' => has_t = true,
                            b'f' => has_f = true,
                            _ => {}
                        }
                    }
                    stack.pop();
                    let op = stack.pop().unwrap();
                    let res = match op {
                        b'!' => !has_t,
                        b'&' => !has_f,
                        b'|' => has_t,
                        _ => unreachable!()
                    };
                    stack.push(if res { b't' } else { b'f' });
                }
                _ => {}
            }
        }
        stack[0] == b't'
    }
}