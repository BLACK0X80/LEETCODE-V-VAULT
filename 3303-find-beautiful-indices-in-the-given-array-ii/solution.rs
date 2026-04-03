impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        fn kmp_search(text: &[u8], pat: &[u8]) -> Vec<usize> {
            let m = pat.len();
            let mut fail = vec![0usize; m];
            let mut j = 0usize;
            for i in 1..m {
                while j > 0 && pat[i] != pat[j] { j = fail[j-1]; }
                if pat[i] == pat[j] { j += 1; }
                fail[i] = j;
            }
            let mut res = vec![];
            j = 0;
            for (i, &c) in text.iter().enumerate() {
                while j > 0 && c != pat[j] { j = fail[j-1]; }
                if c == pat[j] { j += 1; }
                if j == m { res.push(i + 1 - m); j = fail[j-1]; }
            }
            res
        }
        let s = s.as_bytes();
        let pos_a = kmp_search(s, a.as_bytes());
        let pos_b = kmp_search(s, b.as_bytes());
        let k = k as usize;
        let mut res = vec![];
        let mut bi = 0usize;
        for &i in &pos_a {
            while bi < pos_b.len() && pos_b[bi] + k < i { bi += 1; }
            if bi < pos_b.len() && pos_b[bi] <= i + k {
                res.push(i as i32);
            }
        }
        res
    }
}
