use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut memo = HashMap::new();
        Self::solve(s1.as_bytes(), s2.as_bytes(), &mut memo)
    }

    fn solve(s1: &[u8], s2: &[u8], memo: &mut HashMap<(Vec<u8>, Vec<u8>), bool>) -> bool {
        if s1 == s2 { return true; }

        let mut freq = [0i32; 26];
        for (&a, &b) in s1.iter().zip(s2.iter()) {
            freq[(a - b'a') as usize] += 1;
            freq[(b - b'a') as usize] -= 1;
        }
        if freq.iter().any(|&x| x != 0) { return false; }

        let key = (s1.to_vec(), s2.to_vec());
        if let Some(&res) = memo.get(&key) { return res; }

        let n = s1.len();
        for i in 1..n {
            let res =
                (Self::solve(&s1[..i], &s2[..i], memo) && Self::solve(&s1[i..], &s2[i..], memo))
             || (Self::solve(&s1[..i], &s2[n-i..], memo) && Self::solve(&s1[i..], &s2[..n-i], memo));
            if res {
                memo.insert(key, true);
                return true;
            }
        }

        memo.insert(key, false);
        false
    }
}
