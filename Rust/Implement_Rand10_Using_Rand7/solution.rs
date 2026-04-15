impl Solution {
    pub fn rand10() -> i32 {
        loop {
            let idx = (rand7() - 1) * 7 + (rand7() - 1);
            if idx < 40 { return idx % 10 + 1; }
        }
    }
}