impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut black = [0i32; 2];
        for s in &students { black[*s as usize] += 1; }
        for s in &sandwiches {
            if black[*s as usize] == 0 { return black[0] + black[1]; }
            black[*s as usize] -= 1;
        }
        0
    }
}
