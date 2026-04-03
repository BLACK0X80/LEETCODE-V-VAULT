impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut ans = vec![0i32; n];
        let mut stack: Vec<i32> = Vec::new();

        for i in (0..n).rev() {
            let mut count = 0;
            while let Some(&top) = stack.last() {
                count += 1;
                if top >= heights[i] { break; }
                stack.pop();
            }
            ans[i] = count;
            stack.push(heights[i]);
        }

        ans
    }
}
