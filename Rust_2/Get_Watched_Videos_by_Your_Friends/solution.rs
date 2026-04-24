use std::collections::{VecDeque, HashMap, HashSet};

impl Solution {
    pub fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((id as usize, 0));
        visited.insert(id as usize);
        let mut freq: HashMap<&str, i32> = HashMap::new();
        while let Some((u, d)) = queue.pop_front() {
            if d == level {
                for v in &watched_videos[u] { *freq.entry(v).or_insert(0) += 1; }
            } else {
                for &f in &friends[u] {
                    if visited.insert(f as usize) { queue.push_back((f as usize, d + 1)); }
                }
            }
        }
        let mut res: Vec<(&str, i32)> = freq.into_iter().collect();
        res.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        res.into_iter().map(|(v, _)| v.to_string()).collect()
    }
}