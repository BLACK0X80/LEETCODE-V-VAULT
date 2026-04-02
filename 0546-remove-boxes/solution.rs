impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut memo = vec![vec![vec![-1i32; n]; n]; n];
        Self::dp(&boxes, 0, n as i32 - 1, 0, &mut memo)
    }

    fn dp(boxes: &[i32], l: i32, r: i32, k: i32, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if l > r { return 0; }
        let (l, r, k) = (l as usize, r as usize, k as usize);
        if memo[l][r][k] != -1 { return memo[l][r][k]; }

        let mut res = Self::dp(boxes, l as i32, r as i32 - 1, 0, memo) + (k + 1) as i32 * (k + 1) as i32;

        for i in l..r {
            if boxes[i] == boxes[r] {
                res = res.max(
                    Self::dp(boxes, l as i32, i as i32, k as i32 + 1, memo) +
                    Self::dp(boxes, i as i32 + 1, r as i32 - 1, 0, memo)
                );
            }
        }

        memo[l][r][k] = res;
        res
    }
}
