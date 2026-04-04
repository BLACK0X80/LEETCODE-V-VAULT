impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let num: i64 = n.parse().unwrap();
        let len = n.len();
        let mut candidates = vec![
            10i64.pow(len as u32 - 1) - 1,
            10i64.pow(len as u32) + 1,
        ];
        let half: i64 = n[..((len+1)/2)].parse().unwrap();
        for diff in [-1i64, 0, 1] {
            let h = half + diff;
            let s = h.to_string();
            let rev: String = s.chars().rev().collect();
            let pal: String = if len % 2 == 0 { format!("{}{}", s, rev) } else { format!("{}{}", s, &rev[1..]) };
            candidates.push(pal.parse().unwrap());
        }
        candidates.into_iter()
            .filter(|&c| c != num)
            .min_by_key(|&c| (c - num).abs() * 10 + if c < num { 1 } else { 2 })
            .unwrap().to_string()
    }
}
