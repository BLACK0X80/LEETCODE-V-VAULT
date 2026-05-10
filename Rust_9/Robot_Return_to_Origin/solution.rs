impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let b = moves.as_bytes();
        let r = b.iter().filter(|&&c| c == b'R').count();
        let u = b.iter().filter(|&&c| c == b'U').count();
        let l = b.len() - r - u - b.iter().filter(|&&c| c == b'D').count();
        r == l && u == b.len() - r - l - u
    }
}