impl Solution {
    pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
        let n = receiver.len();
        let log = 34usize;
        let mut jump = vec![vec![0usize; n]; log];
        let mut score = vec![vec![0i64; n]; log];

        for i in 0..n {
            jump[0][i] = receiver[i] as usize;
            score[0][i] = receiver[i] as i64;
        }

        for j in 1..log {
            for i in 0..n {
                let mid = jump[j-1][i];
                jump[j][i] = jump[j-1][mid];
                score[j][i] = score[j-1][i] + score[j-1][mid];
            }
        }

        let mut ans = 0i64;
        for i in 0..n {
            let mut cur = i;
            let mut total = i as i64;
            let mut rem = k;
            for j in 0..log {
                if rem & (1 << j) != 0 {
                    total += score[j][cur];
                    cur = jump[j][cur];
                }
            }
            ans = ans.max(total);
        }
        ans
    }
}