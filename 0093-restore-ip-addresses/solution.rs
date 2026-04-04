impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let bytes = s.as_bytes();
        Self::backtrack(bytes, 0, 0, String::new(), &mut result);
        result
    }
    fn backtrack(s: &[u8], idx: usize, parts: usize, current: String, result: &mut Vec<String>) {
        if parts == 4 {
            if idx == s.len() {
                result.push(current[..current.len()-1].to_string());
            }
            return;
        }
        let mut num = 0;
        for i in idx..(idx + 3).min(s.len()) {
            num = num * 10 + (s[i] - b'0') as i32;
            if num > 255 { break; }
            let mut next = current.clone();
            next.push_str(&num.to_string());
            next.push('.');
            Self::backtrack(s, i + 1, parts + 1, next, result);
            if s[idx] == b'0' { break; }
        }
    }
}
