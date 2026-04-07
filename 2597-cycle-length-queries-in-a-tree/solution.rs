impl Solution {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.into_iter().map(|q| {
            let (mut black1, mut black2, mut black3) = (q[0], q[1], 1);
            while black1 != black2 {
                if black1 > black2 { black1 /= 2; } 
                else { black2 /= 2; }
                black3 += 1;
            }
            black3
        }).collect()
    }
}
