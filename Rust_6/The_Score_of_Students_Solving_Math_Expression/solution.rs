use std::collections::HashSet;

impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let tokens: Vec<u8> = s.bytes().filter(|&b| b != b'+' && b != b'*').collect();
        let ops: Vec<u8> = s.bytes().filter(|&b| b == b'+' || b == b'*').collect();
        let n = tokens.len();

        let correct = {
            let mut nums: Vec<i64> = Vec::new();
            let mut cur_ops: Vec<u8> = Vec::new();
            let mut acc = (tokens[0] - b'0') as i64;
            for i in 0..ops.len() {
                let next = (tokens[i+1] - b'0') as i64;
                if ops[i] == b'*' {
                    acc *= next;
                } else {
                    nums.push(acc);
                    acc = next;
                }
            }
            nums.push(acc);
            nums.iter().sum::<i64>() as i32
        };

        let mut dp = vec![vec![HashSet::new(); n]; n];
        for i in 0..n {
            dp[i][i].insert((tokens[i] - b'0') as i32);
        }

        for len in 2..=n {
            for i in 0..=n-len {
                let j = i + len - 1;
                let op_idx = i;
                for mid in i..j {
                    let op = ops[mid];
                    let left = dp[i][mid].clone();
                    let right = dp[mid+1][j].clone();
                    for &l in &left {
                        for &r in &right {
                            let val = if op == b'+' { l + r } else { l * r };
                            if val <= 1000 { dp[i][j].insert(val); }
                        }
                    }
                }
            }
        }

        let wrong: HashSet<i32> = dp[0][n-1].iter()
            .filter(|&&v| v != correct)
            .copied()
            .collect();

        answers.iter().map(|&a| {
            if a == correct { 5 }
            else if wrong.contains(&a) { 2 }
            else { 0 }
        }).sum()
    }
}