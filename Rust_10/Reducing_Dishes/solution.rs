impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let (mut ans, mut total) = (0, 0);
        for s in satisfaction.iter().rev() {
            total += s;
            if total <= 0 { break; }
            ans += total;
        }
        ans
    }
}