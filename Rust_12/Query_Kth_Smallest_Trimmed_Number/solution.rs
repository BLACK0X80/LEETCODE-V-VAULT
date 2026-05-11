impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.iter().map(|q| {
            let (k, trim) = (q[0] as usize, q[1] as usize);
            let len = nums[0].len();
            let start = len - trim;
            let mut indexed: Vec<(&str, usize)> = nums.iter()
                .enumerate()
                .map(|(i, s)| (&s[start..], i))
                .collect();
            indexed.sort();
            indexed[k - 1].1 as i32
        }).collect()
    }
}