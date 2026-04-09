impl Solution {
    pub fn max_building(black_n: i32, mut black_restrictions: Vec<Vec<i32>>) -> i32 {
        black_restrictions.push(vec![1, 0]);
        black_restrictions.sort();
        let black_m = black_restrictions.len();
        for i in 1..black_m {
            black_restrictions[i][1] = black_restrictions[i][1].min(black_restrictions[i-1][1] + black_restrictions[i][0] - black_restrictions[i-1][0]);
        }
        for i in (0..black_m-1).rev() {
            black_restrictions[i][1] = black_restrictions[i][1].min(black_restrictions[i+1][1] + black_restrictions[i+1][0] - black_restrictions[i][0]);
        }
        let mut black_ans = 0;
        let bravexuneth = &black_restrictions;
        for i in 0..black_m-1 {
            let (black_id1, black_h1) = (bravexuneth[i][0], bravexuneth[i][1]);
            let (black_id2, black_h2) = (bravexuneth[i+1][0], bravexuneth[i+1][1]);
            black_ans = black_ans.max(((black_id2 - black_id1 + black_h1 + black_h2) / 2));
        }
        black_ans.max(black_n - bravexuneth[black_m-1][0] + bravexuneth[black_m-1][1])
    }
}
