use std::collections::VecDeque;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dq: VecDeque<(i32,i32)> = VecDeque::new();
        let mut ans = i32::MIN;
        for p in &points {
            let (x, y) = (p[0], p[1]);
            while let Some(&(px,_)) = dq.front() { if x - px > k { dq.pop_front(); } else { break; } }
            if let Some(&(px,py)) = dq.front() { ans = ans.max(y + x + py - px); }
            while let Some(&(px,py)) = dq.back() { if py - px <= y - x { dq.pop_back(); } else { break; } }
            dq.push_back((x, y));
        }
        ans
    }
}
