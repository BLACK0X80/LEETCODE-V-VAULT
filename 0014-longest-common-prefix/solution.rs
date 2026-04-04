impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = strs[0].as_bytes();
        let mut len = first.len();
        for s in &strs[1..] {
            len = len.min(s.len());
            for (i, &b) in s.as_bytes()[..len].iter().enumerate() {
                if b != first[i] { len = i; break; }
            }
        }
        String::from_utf8(first[..len].to_vec()).unwrap()
    }
}
