impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = vec![0; nums.len() + 1];
        for n in &nums { freq[*n as usize] += 1; }
        let (mut dup, mut miss) = (0, 0);
        for i in 1..=nums.len() { if freq[i] == 2 { dup = i as i32; } else if freq[i] == 0 { miss = i as i32; } }
        vec![dup, miss]
    }
}
