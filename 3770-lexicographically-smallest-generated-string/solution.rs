impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let (n, m) = (s1.len(), s2.len());
        let len = n + m - 1;
        let mut word = vec![b'a'; len];
        let mut forced = vec![false; len];

        for i in 0..n {
            if s1[i] == b'T' {
                for j in 0..m {
                    if forced[i+j] && word[i+j] != s2[j] { return String::new(); }
                    word[i+j] = s2[j];
                    forced[i+j] = true;
                }
            }
        }

        let mut fail = vec![0usize; m];
        let mut j = 0usize;
        for i in 1..m {
            while j > 0 && s2[i] != s2[j] { j = fail[j-1]; }
            if s2[i] == s2[j] { j += 1; }
            fail[i] = j;
        }

        for i in 0..n {
            if s1[i] == b'F' {
                let mut eq = true;
                for j in 0..m {
                    if word[i+j] != s2[j] { eq = false; break; }
                }
                if eq {
                    let mut fixed = false;
                    for j in (0..m).rev() {
                        if !forced[i+j] && word[i+j] < b'z' {
                            word[i+j] += 1;
                            fixed = true;
                            break;
                        }
                    }
                    if !fixed { return String::new(); }
                }
            }
        }

        for i in 0..n {
            let eq = word[i..i+m] == s2[..];
            if s1[i] == b'T' && !eq { return String::new(); }
            if s1[i] == b'F' && eq { return String::new(); }
        }

        String::from_utf8(word).unwrap()
    }
}
