impl Solution {
    pub fn shortest_matching_substring(s: String, p: String) -> i32 {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let stars: Vec<usize> = p.iter().enumerate().filter(|&(_,&c)| c == b'*').map(|(i,_)| i).collect();
        let (i1, i2) = (stars[0], stars[1]);
        let p1 = &p[..i1];
        let p2 = &p[i1+1..i2];
        let p3 = &p[i2+1..];

        fn kmp_fail(pat: &[u8]) -> Vec<usize> {
            let m = pat.len();
            let mut f = vec![0usize; m];
            let mut j = 0usize;
            for i in 1..m {
                while j > 0 && pat[i] != pat[j] { j = f[j-1]; }
                if pat[i] == pat[j] { j += 1; }
                f[i] = j;
            }
            f
        }

        fn kmp_search(text: &[u8], pat: &[u8]) -> Vec<usize> {
            if pat.is_empty() { return (0..=text.len()).collect(); }
            let f = kmp_fail(pat);
            let mut res = vec![];
            let mut j = 0usize;
            for (i, &c) in text.iter().enumerate() {
                while j > 0 && c != pat[j] { j = f[j-1]; }
                if c == pat[j] { j += 1; }
                if j == pat.len() { res.push(i + 1 - pat.len()); j = f[j-1]; }
            }
            res
        }

        let pos1 = kmp_search(s, p1);
        let pos2 = kmp_search(s, p2);
        let pos3 = kmp_search(s, p3);

        let n = s.len();
        let mut ans = i32::MAX;
        let (mut j2, mut j3) = (0usize, 0usize);

        for &start in &pos1 {
            let after1 = start + p1.len();
            while j2 < pos2.len() && pos2[j2] < after1 { j2 += 1; }
            if j2 >= pos2.len() { break; }
            let after2 = pos2[j2] + p2.len();
            while j3 < pos3.len() && pos3[j3] < after2 { j3 += 1; }
            if j3 >= pos3.len() { break; }
            let end = pos3[j3] + p3.len();
            ans = ans.min((end - start) as i32);
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}