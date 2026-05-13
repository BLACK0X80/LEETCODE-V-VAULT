impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut freq = [0i32; 100001];
        for c in costs { freq[c as usize] += 1; }
        let mut ans = 0;
        for price in 1..=100000 {
            if coins < price as i32 { break; }
            let take = freq[price].min(coins / price as i32);
            ans += take;
            coins -= take * price as i32;
        }
        ans
    }
}