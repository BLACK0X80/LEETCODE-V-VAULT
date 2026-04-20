impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let b = text.as_bytes();
        let n = b.len();
        
        for l in 1..=n/2 {
            if b[..l] == b[n-l..] {
                return 2 + Solution::longest_decomposition(
                    String::from_utf8(b[l..n-l].to_vec()).unwrap()
                );
            }
        }
        
        if n > 0 { 1 } else { 0 }
    }
}