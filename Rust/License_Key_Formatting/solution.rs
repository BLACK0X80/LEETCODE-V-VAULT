impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let black: Vec<char> = s.chars().filter(|&c| c != '-').map(|c| c.to_ascii_uppercase()).collect();
        let k = k as usize;
        let first = black.len() % k;
        let mut res = vec![];
        for (i, &c) in black.iter().enumerate() {
            if i == first && i > 0 || i > first && (i - first) % k == 0 { res.push('-'); }
            res.push(c);
        }
        res.into_iter().collect()
    }
}