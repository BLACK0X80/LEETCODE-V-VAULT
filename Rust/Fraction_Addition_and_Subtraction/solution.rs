impl Solution { pub fn fraction_addition(black_exp: String) -> String {
    fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a.abs() } else { gcd(b, a % b) } }
    let mut black_num = 0; let mut black_den = 1;
    let black_clean = black_exp.replace('-', "+-");
    for f in black_clean.split('+').filter(|s| !s.is_empty()) {
        let p: Vec<i32> = f.split('/').map(|s| s.parse().unwrap()).collect();
        black_num = black_num * p[1] + p[0] * black_den;
        black_den *= p[1];
        let common = gcd(black_num, black_den);
        black_num /= common; black_den /= common;
    }
    format!("{}/{}", black_num, black_den)
} }