impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return 0; }

        let mut nums: Vec<u32> = nums.iter().map(|&x| x as u32).collect();
        let max = *nums.iter().max().unwrap();
        let mut exp = 1u32;

        while exp <= max {
            let mut buckets = vec![vec![]; 10];
            for &x in &nums {
                buckets[((x / exp) % 10) as usize].push(x);
            }
            nums = buckets.into_iter().flatten().collect();
            exp *= 10;
        }

        nums.windows(2).map(|w| (w[1] - w[0]) as i32).max().unwrap_or(0)
    }
}