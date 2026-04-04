impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let min = *nums.iter().min().unwrap();
        let mut nums: Vec<u32> = nums.iter().map(|&x| (x - min) as u32).collect();
        let mut exp = 1u32;
        let max = *nums.iter().max().unwrap();

        while exp <= max {
            let mut buckets = vec![vec![]; 10];
            for &x in &nums {
                buckets[((x / exp) % 10) as usize].push(x);
            }
            nums = buckets.into_iter().flatten().collect();
            exp *= 10;
        }

        nums.iter().map(|&x| x as i32 + min).collect()
    }
}
