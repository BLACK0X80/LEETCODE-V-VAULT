impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut max_left = vec![0; n];
        let mut min_right = vec![0; n];
        max_left[0] = arr[0];
        for i in 1..n { max_left[i] = max_left[i-1].max(arr[i]); }
        min_right[n-1] = arr[n-1];
        for i in (0..n-1).rev() { min_right[i] = min_right[i+1].min(arr[i]); }
        let mut chunks = 1;
        for i in 0..n-1 { if max_left[i] <= min_right[i+1] { chunks += 1; } }
        chunks
    }
}