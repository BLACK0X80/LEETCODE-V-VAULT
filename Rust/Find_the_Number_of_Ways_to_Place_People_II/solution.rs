impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let n = points.len();
        let mut ans = 0;

        for i in 0..n {
            let mut max_y = i32::MIN;
            for j in i+1..n {
                if points[j][0] >= points[i][0] && points[j][1] <= points[i][1] {
                    if points[j][1] > max_y {
                        ans += 1;
                        max_y = points[j][1];
                    }
                }
            }
        }
        ans
    }
}