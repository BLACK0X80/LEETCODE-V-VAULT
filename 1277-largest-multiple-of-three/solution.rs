impl Solution {
    pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
        digits.sort_by(|a,b| b.cmp(a));
        let sum: i32 = digits.iter().sum();
        let rem = (sum % 3) as usize;
        let remove_one = |digits: &mut Vec<i32>, r: usize| {
            if let Some(i) = digits.iter().rposition(|&d| d % 3 == r as i32) { digits.remove(i); return true; }
            false
        };
        let mut d = digits.clone();
        if rem != 0 {
            let mut d1 = d.clone();
            if remove_one(&mut d1, rem) { d = d1; }
            else {
                remove_one(&mut d, (3 - rem) % 3);
                remove_one(&mut d, (3 - rem) % 3);
            }
        }
        if d.is_empty() { return String::new(); }
        if d[0] == 0 { return "0".to_string(); }
        d.iter().map(|x| x.to_string()).collect()
    }
}
