impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let mut ans = vec![-1i32; queries.len()];
        let mut pending: Vec<Vec<(i32, usize)>> = vec![vec![]; n];

        for (i, q) in queries.iter().enumerate() {
            let (mut a, mut b) = (q[0] as usize, q[1] as usize);
            if a > b { std::mem::swap(&mut a, &mut b); }
            if a == b || heights[a] < heights[b] {
                ans[i] = b as i32;
            } else {
                pending[b].push((heights[a].max(heights[b]), i));
            }
        }

        let mut stack: Vec<(i32, usize)> = vec![];

        for i in (0..n).rev() {
            for &(need, qi) in &pending[i] {
                
                if stack.is_empty() || stack[0].0 <= need { continue; }
                let mut lo = 0usize;
                let mut hi = stack.len();
                while lo + 1 < hi {
                    let mid = (lo + hi) / 2;
                    if stack[mid].0 > need { lo = mid; } else { hi = mid; }
                }
                ans[qi] = stack[lo].1 as i32;
            }
            while !stack.is_empty() && stack.last().unwrap().0 <= heights[i] {
                stack.pop();
            }
            stack.push((heights[i], i));
        }
        ans
    }
}