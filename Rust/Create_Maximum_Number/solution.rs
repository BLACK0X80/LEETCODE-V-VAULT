impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let m = nums1.len();
        let n = nums2.len();
        let k = k as usize;
        let mut best = vec![0i32; k];

        let lo = if k > n { k - n } else { 0 };
        let hi = k.min(m);

        for i in lo..=hi {
            let j = k - i;
            if j > n { continue; }

            let sub1 = max_subsequence(&nums1, i);
            let sub2 = max_subsequence(&nums2, j);
            let merged = merge(&sub1, &sub2);

            if merged > best {
                best = merged;
            }
        }

        best
    }
}

fn max_subsequence(nums: &[i32], k: usize) -> Vec<i32> {
    let n = nums.len();
    let mut stack: Vec<i32> = Vec::new();
    let mut drop = n - k;

    for &num in nums {
        while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < num {
            stack.pop();
            drop -= 1;
        }
        stack.push(num);
    }

    stack[..k].to_vec()
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i..] >= b[j..] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}