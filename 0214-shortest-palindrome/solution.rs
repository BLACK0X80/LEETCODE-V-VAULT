impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let rev: String = s.chars().rev().collect();
        let combined = format!("{}#{}", s, rev);
        let b = combined.as_bytes();
        let n = b.len();
        let mut fail = vec![0usize; n];

        for i in 1..n {
            let mut j = fail[i - 1];
            while j > 0 && b[i] != b[j] { j = fail[j - 1]; }
            if b[i] == b[j] { j += 1; }
            fail[i] = j;
        }

        let palindrome_len = fail[n - 1];
        let suffix: String = s[palindrome_len..].chars().rev().collect();
        format!("{}{}", suffix, s)
    }
}
