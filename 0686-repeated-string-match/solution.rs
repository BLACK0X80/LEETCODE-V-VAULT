impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let times = (b.len() + a.len() - 1) / a.len();
        for extra in 0..=1 {
            let black = a.repeat(times + extra);
            if black.contains(&b as &str) { return (times + extra) as i32; }
        }
        -1
    }
}
