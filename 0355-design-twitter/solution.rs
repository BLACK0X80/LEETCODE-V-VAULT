use std::collections::{HashMap, HashSet, BinaryHeap};

struct Twitter {
    time: i32,
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    follows: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self { Twitter { time: 0, tweets: HashMap::new(), follows: HashMap::new() } }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.entry(user_id).or_default().push((self.time, tweet_id));
        self.time += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let empty = HashSet::new();
        let following = self.follows.get(&user_id).unwrap_or(&empty);
        let users: Vec<i32> = std::iter::once(user_id).chain(following.iter().copied()).collect();
        for uid in users {
            if let Some(ts) = self.tweets.get(&uid) {
                for &(t, tid) in ts { heap.push((t, tid)); }
            }
        }
        (0..10).filter_map(|_| heap.pop().map(|(_, tid)| tid)).collect()
    }

    fn follow(&mut self, follower: i32, followee: i32) {
        self.follows.entry(follower).or_default().insert(followee);
    }

    fn unfollow(&mut self, follower: i32, followee: i32) {
        if let Some(s) = self.follows.get_mut(&follower) { s.remove(&followee); }
    }
}
