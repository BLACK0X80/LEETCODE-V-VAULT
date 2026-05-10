impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut counts = vec![0i32; n];
        let mut indexed: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        let mut temp = vec![(0i32, 0usize); n];

        merge_sort(&mut indexed, &mut temp, &mut counts, 0, n);
        counts
    }
}

fn merge_sort(
    arr: &mut Vec<(i32, usize)>,
    temp: &mut Vec<(i32, usize)>,
    counts: &mut Vec<i32>,
    left: usize,
    right: usize,
) {
    if right - left <= 1 {
        return;
    }

    let mid = (left + right) / 2;
    merge_sort(arr, temp, counts, left, mid);
    merge_sort(arr, temp, counts, mid, right);

    let mut i = left;
    let mut j = mid;
    let mut k = left;

    while i < mid && j < right {
        if arr[i].0 <= arr[j].0 {
            counts[arr[i].1] += (j - mid) as i32;
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        counts[arr[i].1] += (j - mid) as i32;
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j < right {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }

    arr[left..right].copy_from_slice(&temp[left..right]);
}