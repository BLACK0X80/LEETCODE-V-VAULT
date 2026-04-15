impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let n = s.len();

        let mut has_num = false;
        let mut has_dot = false;
        let mut has_exp = false;

        if i < n && (s[i] == b'+' || s[i] == b'-') {
            i += 1;
        }

        while i < n {
            match s[i] {
                b'0'..=b'9' => {
                    has_num = true;
                }
                b'.' => {
                    if has_dot || has_exp {
                        return false;
                    }
                    has_dot = true;
                }
                b'e' | b'E' => {
                    if has_exp || !has_num {
                        return false;
                    }
                    has_exp = true;
                    has_num = false;
                    if i + 1 < n && (s[i + 1] == b'+' || s[i + 1] == b'-') {
                        i += 1;
                    }
                }
                _ => return false,
            }
            i += 1;
        }

        has_num
    }
}