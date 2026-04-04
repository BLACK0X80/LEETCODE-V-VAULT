impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            let s = s.to_lowercase();
            let at = s.find('@').unwrap();
            let name: Vec<char> = s[..at].chars().collect();
            format!("{}*****{}{}", name[0], name[name.len()-1], &s[at..])
        } else {
            let black: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let last4: String = black[black.len()-4..].iter().collect();
            let prefix = match black.len() - 10 {
                0 => "***-***-".to_string(),
                1 => "+*-***-***-".to_string(),
                2 => "+**-***-***-".to_string(),
                _ => "+***-***-***-".to_string(),
            };
            format!("{}{}", prefix, last4)
        }
    }
}
