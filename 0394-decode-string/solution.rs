impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut idx = 0;
        Self::decode(s.as_bytes(), &mut idx)
    }
    fn decode(s: &[u8], idx: &mut usize) -> String {
        let mut result = String::new();
        let mut num = 0;
        while *idx < s.len() {
            match s[*idx] {
                b'0'..=b'9' => {
                    num = num * 10 + (s[*idx] - b'0') as i32;
                    *idx += 1;
                }
                b'[' => {
                    *idx += 1;
                    let inner = Self::decode(s, idx);
                    for _ in 0..num { result.push_str(&inner); }
                    num = 0;
                    *idx += 1;
                }
                b']' => break,
                c => {
                    result.push(c as char);
                    *idx += 1;
                }
            }
        }
        result
    }
}
