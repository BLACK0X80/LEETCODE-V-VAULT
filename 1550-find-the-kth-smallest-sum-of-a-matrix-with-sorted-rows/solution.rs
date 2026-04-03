impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut sums = mat[0].clone();
        sums.truncate(k);
        for row in &mat[1..] {
            let mut next = Vec::with_capacity(sums.len() * row.len());
            for &s in &sums {
                for &x in row { next.push(s + x); }
            }
            next.sort_unstable();
            next.truncate(k);
            sums = next;
        }
        sums[k - 1]
    }
}
