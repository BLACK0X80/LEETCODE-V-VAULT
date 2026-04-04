impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        tickets.iter().enumerate().map(|(i, &t)| {
            if i <= k { t.min(tickets[k]) }
            else { t.min(tickets[k] - 1) }
        }).sum()
    }
}
