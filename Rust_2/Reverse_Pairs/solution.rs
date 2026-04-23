impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        let mut temp = vec![0i64; n];
        Self::merge_sort(&mut nums, &mut temp, 0, n) as i32
    }
    fn merge_sort(nums: &mut [i32], temp: &mut [i64], l: usize, r: usize) -> usize {
        if r - l <= 1 { return 0; }
        let m = l + (r - l) / 2;
        let mut cnt = Self::merge_sort(nums, temp, l, m) + Self::merge_sort(nums, temp, m, r);
        let mut j = m;
        for i in l..m {
            while j < r && nums[i] as i64 > 2 * nums[j] as i64 { j += 1; }
            cnt += j - m;
        }
        for i in l..r { temp[i] = nums[i] as i64; }
        let (mut i, mut k) = (l, l);
        j = m;
        while i < m && j < r {
            if temp[i] <= temp[j] { nums[k] = temp[i] as i32; i += 1; }
            else { nums[k] = temp[j] as i32; j += 1; }
            k += 1;
        }
        while i < m { nums[k] = temp[i] as i32; i += 1; k += 1; }
        while j < r { nums[k] = temp[j] as i32; j += 1; k += 1; }
        cnt
    }
}