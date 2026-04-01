impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = vec![];
        let mut max_area = 0;
        let n = heights.len();

        for i in 0..=n {
            let h = if i == n { 0 } else { heights[i] };

            while let Some(&top) = stack.last() {
                if h >= heights[top] { break; }
                stack.pop();
                let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                max_area = max_area.max(heights[top] * width as i32);
            }

            stack.push(i);
        }

        max_area
    }
}
