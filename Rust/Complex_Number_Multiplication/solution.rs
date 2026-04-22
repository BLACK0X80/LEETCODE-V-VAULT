impl Solution { pub fn complex_number_multiply(black_n1: String, black_n2: String) -> String {
    let parse = |s: String| -> (i32, i32) {
        let parts: Vec<&str> = s[..s.len()-1].split('+').collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    };
    let (a, b) = parse(black_n1); let (c, d) = parse(black_n2);
    format!("{}+{}i", a * c - b * d, a * d + b * c)
} }