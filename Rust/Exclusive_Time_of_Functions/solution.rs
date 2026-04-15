impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut res = vec![0i32; n as usize];
        let mut stack: Vec<(usize, i32)> = vec![];
        let mut prev = 0;
        for log in &logs {
            let p: Vec<&str> = log.split(':').collect();
            let id = p[0].parse::<usize>().unwrap();
            let t = p[2].parse::<i32>().unwrap();
            if p[1] == "start" {
                if let Some(&(top, _)) = stack.last() {
                    res[top] += t - prev;
                }
                stack.push((id, t));
                prev = t;
            } else {
                stack.pop();
                res[id] += t - prev + 1;
                prev = t + 1;
            }
        }
        res
    }
}