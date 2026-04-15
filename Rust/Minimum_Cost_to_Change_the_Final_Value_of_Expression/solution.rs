impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let mut stack: Vec<(u8, i32)> = Vec::new();

        for b in expression.bytes() {
            match b {
                b'(' | b'&' | b'|' => stack.push((b, 0)),
                _ => {
                    let mut p = if b == b')' {
                        let top = stack.pop().unwrap();
                        stack.pop();
                        top
                    } else {
                        (b, 1)
                    };

                    while matches!(stack.last().map(|x| x.0), Some(b'&') | Some(b'|')) {
                        let (op, _) = stack.pop().unwrap();
                        let (v2, c2) = p;
                        let (v1, c1) = stack.pop().unwrap();

                        p = match (op, v1, v2) {
                            (b'&', b'1', b'1') => (b'1', c1.min(c2)),
                            (b'&', b'0', b'0') => (b'0', 1 + c1.min(c2)),
                            (b'&', _, _)       => (b'0', 1),
                            (b'|', b'0', b'0') => (b'0', c1.min(c2)),
                            (b'|', b'1', b'1') => (b'1', 1 + c1.min(c2)),
                            _                  => (b'1', 1),
                        };
                    }
                    stack.push(p);
                }
            }
        }
        stack[0].1
    }
}