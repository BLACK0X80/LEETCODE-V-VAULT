impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let k = k as usize;
        let n = s.len();
        let b = s.as_bytes();
        let freq: Vec<u8> = (b'a'..=b'z').filter(|&c| b.iter().filter(|&&x| x == c).count() >= k).collect();

        let is_subseq = |sub: &[u8]| -> bool {
            let mut rep = 0;
            let mut j = 0;
            for &c in b {
                if c == sub[j] { j += 1; if j == sub.len() { rep += 1; j = 0; if rep == k { return true; } } }
            }
            false
        };

        let mut queue: Vec<Vec<u8>> = vec![vec![]];
        let mut ans: Vec<u8> = vec![];

        while let Some(cur) = queue.pop() {
            for &c in freq.iter().rev() {
                let mut next = cur.clone();
                next.push(c);
                if is_subseq(&next) {
                    if next.len() > ans.len() || (next.len() == ans.len() && next > ans) {
                        ans = next.clone();
                    }
                    queue.push(next);
                }
            }
        }

        String::from_utf8(ans).unwrap()
    }
}
