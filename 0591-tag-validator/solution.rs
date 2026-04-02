impl Solution {
    pub fn is_valid(code: String) -> bool {
        let mut stack: Vec<String> = vec![];
        let mut i = 0;
        let s = code.as_bytes();
        let n = s.len();

        while i < n {
            if i > 0 && stack.is_empty() { return false; }

            if s[i] == b'<' {
                if s[i..].starts_with(b"<![CDATA[") {
                    if let Some(end) = s[i+9..].windows(3).position(|w| w == b"]]>") {
                        i += 9 + end + 3;
                        continue;
                    } else {
                        return false;
                    }
                } else if i + 1 < n && s[i+1] == b'/' {
                    if let Some(end) = s[i..].iter().position(|&c| c == b'>') {
                        let tag = std::str::from_utf8(&s[i+2..i+end]).unwrap();
                        if !is_valid_tag(tag) { return false; }
                        if stack.last().map(|t| t != tag).unwrap_or(true) { return false; }
                        stack.pop();
                        i += end + 1;
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    if let Some(end) = s[i..].iter().position(|&c| c == b'>') {
                        let tag = std::str::from_utf8(&s[i+1..i+end]).unwrap();
                        if !is_valid_tag(tag) { return false; }
                        stack.push(tag.to_string());
                        i += end + 1;
                        continue;
                    } else {
                        return false;
                    }
                }
            }
            i += 1;
        }
        stack.is_empty()
    }
}

fn is_valid_tag(t: &str) -> bool {
    !t.is_empty() && t.len() <= 9 && t.chars().all(|c| c.is_ascii_uppercase())
}
