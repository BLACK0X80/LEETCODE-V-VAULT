impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0; n];
        let mut stack = vec![];
        for i in 0..n {
            while let Some(&j) = stack.last() {
                if temperatures[i] > temperatures[j] { res[j] = (i - j) as i32; stack.pop(); }
                else { break; }
            }
            stack.push(i);
        }
        res
    }
}
