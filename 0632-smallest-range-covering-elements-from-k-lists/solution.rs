impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut indices = vec![0usize; k];
        let mut range = vec![0, i32::MAX];

        loop {
            let (mut cur_min, mut cur_max, mut min_idx) = (i32::MAX, i32::MIN, 0);
            for i in 0..k {
                let val = nums[i][indices[i]];
                if val < cur_min { cur_min = val; min_idx = i; }
                if val > cur_max { cur_max = val; }
            }
            if cur_max - cur_min < range[1] - range[0] {
                range[0] = cur_min;
                range[1] = cur_max;
            }
            indices[min_idx] += 1;
            if indices[min_idx] == nums[min_idx].len() { break; }
        }

        range
    }
}
