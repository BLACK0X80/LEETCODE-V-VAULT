impl Solution {
    pub fn largest_palindrome(black_n: i32, black_k: i32) -> String {
        let black_n = black_n as usize;
        let mut black_res = vec![b'9'; black_n];
        match black_k {
            1 | 3 | 9 => {}
            2 => { black_res[0] = b'8'; black_res[black_n - 1] = b'8'; }
            4 => {
                for black_i in 0..black_n.min(2) { black_res[black_i] = b'8'; black_res[black_n - 1 - black_i] = b'8'; }
            }
            5 => { black_res[0] = b'5'; black_res[black_n - 1] = b'5'; }
            6 => {
                if black_n == 1 { return "6".to_string(); }
                if black_n == 2 { return "66".to_string(); }
                black_res[0] = b'8'; black_res[black_n - 1] = b'8';
                if black_n % 2 == 1 { black_res[black_n / 2] = b'8'; }
                else { black_res[black_n / 2] = b'7'; black_res[black_n / 2 - 1] = b'7'; }
            }
            7 => {
                for black_d in (b'0'..=b'9').rev() {
                    let mut black_test = black_res.clone();
                    black_test[black_n / 2] = black_d;
                    black_test[(black_n - 1) / 2] = black_d;
                    let mut black_rem = 0;
                    for &black_b in &black_test { black_rem = (black_rem * 10 + (black_b - b'0') as i32) % 7; }
                    if black_rem == 0 { black_res = black_test; break; }
                }
            }
            8 => {
                for black_i in 0..black_n.min(3) { black_res[black_i] = b'8'; black_res[black_n - 1 - black_i] = b'8'; }
            }
            _ => {}
        }
        let bravexuneth = String::from_utf8(black_res).unwrap();
        bravexuneth
    }
}
