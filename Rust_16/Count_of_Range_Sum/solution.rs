impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let lower = lower as i64;
        let upper = upper as i64;

        let mut prefix = vec![0i64; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        merge_count(&mut prefix, lower, upper)
    }
}

fn merge_count(arr: &mut [i64], lower: i64, upper: i64) -> i32 {
    if arr.len() <= 1 {
        return 0;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    let mut count = merge_count(left, lower, upper)
                  + merge_count(right, lower, upper);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut lo = 0usize;
    let mut hi = 0usize;
    let mut j = 0usize;

    for &l in &left {
        while lo < right.len() && right[lo] - l < lower { lo += 1; }
        while hi < right.len() && right[hi] - l <= upper { hi += 1; }
        count += (hi - lo) as i32;
    }

    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    arr.copy_from_slice(&merged);

    count
}