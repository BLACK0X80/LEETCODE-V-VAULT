impl Solution {
    pub fn count_good_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans = 0i64;
        let mut bit_pos: Vec<Vec<usize>> = vec![vec![]; 30];
        let mut pos: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();
        let mut count: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

        for i in 0..n {
            for j in 0..30 {
                if nums[i] & (1 << j) != 0 {
                    bit_pos[j].push(i);
                }
            }
            pos.entry(nums[i]).or_default().push(i);
        }

        for index in 0..n {
            let val = nums[index];
            let mut left = 0usize;
            let mut right = n - 1;

            for j in 0..30 {
                if val & (1 << j) == 0 {
                    let bp = &bit_pos[j];
                    match bp.binary_search(&index) {
                        Ok(k) | Err(k) => {
                            if k < bp.len() { right = right.min(bp[k] - 1 + (if bp[k] == 0 { 1 } else { 0 })); }
                            if k < bp.len() && bp[k] > index { right = right.min(bp[k] - 1); }
                            if k > 0 { left = left.max(bp[k-1] + 1); }
                        }
                    }
                }
            }

            let right_dist = right - index + 1;
            let cnt = *count.get(&val).unwrap_or(&0);
            if cnt > 0 {
                let last_seen = pos[&val][cnt - 1];
                left = left.max(last_seen + 1);
            }
            let left_dist = index - left + 1;
            ans += (right_dist * left_dist) as i64;
            *count.entry(val).or_insert(0) += 1;
        }
        ans
    }
}
