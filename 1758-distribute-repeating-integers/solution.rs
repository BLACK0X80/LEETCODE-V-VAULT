impl Solution {
    pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
        let mut cnt = [0i32; 1001];
        for x in nums { cnt[x as usize] += 1; }
        let mut b: Vec<i32> = cnt.into_iter().filter(|&c| c > 0).collect();
        b.sort_unstable_by(|a, b| b.cmp(a));
        quantity.sort_unstable_by(|a, b| b.cmp(a));

        fn dfs(b: &mut Vec<i32>, q: &[i32], i: usize) -> bool {
            if i == q.len() { return true; }
            let v = q[i];
            for j in 0..b.len() {
                if b[j] >= v {
                    b[j] -= v;
                    if dfs(b, q, i + 1) { return true; }
                    b[j] += v;
                    if b[j] == v { return false; }
                }
            }
            false
        }

        dfs(&mut b, &quantity, 0)
    }
}
