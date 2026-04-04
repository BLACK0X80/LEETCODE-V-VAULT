impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let n = s.len();
            let doubled = s.repeat(2);
            (0..n).map(|i| &doubled[i..i+n]).min().unwrap().to_string()
        } else {
            let mut b = s.into_bytes();
            b.sort();
            String::from_utf8(b).unwrap()
        }
    }
}
