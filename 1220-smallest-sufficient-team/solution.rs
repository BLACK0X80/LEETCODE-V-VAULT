use std::collections::HashMap;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let n = req_skills.len();
        let skill_idx: HashMap<&str, usize> = req_skills.iter().enumerate()
            .map(|(i, s)| (s.as_str(), i)).collect();

        let masks: Vec<usize> = people.iter().map(|p|
            p.iter().fold(0, |acc, s| acc | (1 << skill_idx[s.as_str()]))
        ).collect();

        let full = (1 << n) - 1;
        let mut dp = vec![usize::MAX; full + 1];
        let mut par = vec![(0usize, 0usize); full + 1];
        dp[0] = 0;

        for mask in 0..=full {
            if dp[mask] == usize::MAX { continue; }
            for (i, &pm) in masks.iter().enumerate() {
                let nm = mask | pm;
                if dp[mask] + 1 < dp[nm] {
                    dp[nm] = dp[mask] + 1;
                    par[nm] = (mask, i);
                }
            }
        }

        let mut res = Vec::new();
        let mut cur = full;
        while cur != 0 {
            let (prev, person) = par[cur];
            res.push(person as i32);
            cur = prev;
        }
        res
    }
}
