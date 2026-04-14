impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let m = requests.len();
        let mut max_req = 0;
        let limit = 1usize << m;
        for mask in 0..limit {
            let cnt = mask.count_ones() as i32;
            if cnt <= max_req { continue; }
            let mut balance = vec![0i32; n as usize];
            for i in 0..m {
                if (mask >> i) & 1 == 1 {
                    balance[requests[i][0] as usize] -= 1;
                    balance[requests[i][1] as usize] += 1;
                }
            }
            if balance.iter().all(|&b| b == 0) {
                max_req = cnt;
            }
        }
        max_req
    }
}