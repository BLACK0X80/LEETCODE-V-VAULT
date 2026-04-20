impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let mut can = [[false; 128]; 128];
        for m in &mappings { can[m[0] as usize][m[1] as usize] = true; }
        let s = s.as_bytes();
        let sub = sub.as_bytes();
        let (n, m) = (s.len(), sub.len());
        'outer: for i in 0..=n-m {
            for j in 0..m {
                let sc = s[i+j];
                let subc = sub[j];
                if sc != subc && !can[subc as usize][sc as usize] { continue 'outer; }
            }
            return true;
        }
        false
    }
}