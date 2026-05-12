impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for t in &tokens {
            match t.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(match t.as_str() {
                        "+" => a + b, "-" => a - b, "*" => a * b, _ => a / b,
                    });
                }
                _ => stack.push(t.parse::<i32>().unwrap()),
            }
        }
        stack[0]
    }
}