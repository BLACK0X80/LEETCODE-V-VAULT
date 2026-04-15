use std::collections::HashMap;

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let mut memo = HashMap::new();
        (dp(0, target as i64, x as i64, &mut memo) - 1) as i32
    }
}

fn dp(i: i64, t: i64, x: i64, memo: &mut HashMap<(i64,i64), i64>) -> i64 {
    if t == 0 { return 0; }
    if t == 1 { return if i == 0 { 2 } else { i }; }
    if i >= 40 { return i64::MAX / 2; }
    if let Some(&v) = memo.get(&(i, t)) { return v; }
    let cost = if i == 0 { 2 } else { i };
    let (q, r) = (t / x, t % x);
    let res = (r * cost + dp(i+1, q, x, memo))
        .min((x - r) * cost + dp(i+1, q+1, x, memo));
    memo.insert((i, t), res);
    res
}