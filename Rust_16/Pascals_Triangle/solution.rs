impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut black = vec![vec![1]];
        for b in 1..num_rows as usize {
            let prev = &black[b-1];
            let mut row = vec![1];
            for i in 0..prev.len()-1 { row.push(prev[i]+prev[i+1]); }
            row.push(1);
            black.push(row);
        }
        black
    }
}