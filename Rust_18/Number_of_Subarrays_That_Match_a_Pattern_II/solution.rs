impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = pattern.len();
        let text: Vec<i32> = (1..n).map(|i| nums[i].cmp(&nums[i-1]) as i32).collect();
        let mut fail = vec![0usize; m];
        let mut j = 0;
        for i in 1..m {
            while j > 0 && pattern[i] != pattern[j] { j = fail[j-1]; }
            if pattern[i] == pattern[j] { j += 1; }
            fail[i] = j;
        }
        let mut count = 0;
        j = 0;
        for i in 0..text.len() {
            while j > 0 && text[i] != pattern[j] { j = fail[j-1]; }
            if text[i] == pattern[j] { j += 1; }
            if j == m { count += 1; j = fail[j-1]; }
        }
        count
    }
}