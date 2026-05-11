impl Solution {
    pub fn reformat_date(date: String) -> String {
        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let parts: Vec<&str> = date.split_whitespace().collect();
        let day = parts[0][..parts[0].len()-2].parse::<i32>().unwrap();
        let month = months.iter().position(|&m| m == parts[1]).unwrap() + 1;
        let year = parts[2];
        format!("{}-{:02}-{:02}", year, month, day)
    }
}