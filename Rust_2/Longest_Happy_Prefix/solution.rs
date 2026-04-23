impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut fail = vec![0usize; n];
        for i in 1..n {
            let mut j = fail[i - 1];
            while j > 0 && b[i] != b[j] { j = fail[j - 1]; }
            if b[i] == b[j] { j += 1; }
            fail[i] = j;
        }
        let len = fail[n - 1];
        if len == 0 { return String::new(); }
        s[..len].to_string()
    }
}