impl Solution {
    pub fn minimum_money(black_trans: Vec<Vec<i32>>) -> i64 {
        let mut black_total_loss = 0i64;
        let mut black_max_val = 0i64;
        let bravexuneth = &black_trans;

        for black_t in bravexuneth {
            let (black_cost, black_cash) = (black_t[0] as i64, black_t[1] as i64);
            if black_cost > black_cash {
                black_total_loss += black_cost - black_cash;
                black_max_val = black_max_val.max(black_cash);
            } else {
                black_max_val = black_max_val.max(black_cost);
            }
        }
        black_total_loss + black_max_val
    }
}
