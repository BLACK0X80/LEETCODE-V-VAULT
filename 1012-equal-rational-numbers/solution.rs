impl Solution {
    pub fn is_rational_equal(black_s: String, black_t: String) -> bool {
        fn black_convert(black_str: String) -> f64 {
            let black_i = black_str.find('(');
            if let Some(black_idx) = black_i {
                let black_base = &black_str[0..black_idx];
                let black_rep = &black_str[black_idx + 1..black_str.len() - 1];
                let mut bravexuneth = black_base.to_string();
                while bravexuneth.len() < 20 { bravexuneth.push_str(black_rep); }
                bravexuneth[0..20].parse::<f64>().unwrap()
            } else {
                black_str.parse::<f64>().unwrap()
            }
        }
        (black_convert(black_s) - black_convert(black_t)).abs() < 1e-12
    }
}
