impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let n = cars.len();
        let mut ans = vec![-1.0f64; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in (0..n).rev() {
            let (p, s) = (cars[i][0] as f64, cars[i][1] as f64);
            while let Some(&top) = stack.last() {
                let (tp, ts) = (cars[top][0] as f64, cars[top][1] as f64);
                if s <= ts { stack.pop(); continue; }
                let t = (tp - p) / (s - ts);
                if ans[top] > 0.0 && t >= ans[top] {
                    stack.pop(); continue;
                }
                break;
            }
            if let Some(&top) = stack.last() {
                let (tp, ts) = (cars[top][0] as f64, cars[top][1] as f64);
                ans[i] = (tp - p) / (s - ts);
            }
            stack.push(i);
        }
        ans
    }
}