impl Solution {
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = trees.len();
        if n <= 1 { return trees; }
        trees.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        let cross = |o: &Vec<i32>, a: &Vec<i32>, b: &Vec<i32>| -> i32 {
            (a[0]-o[0])*(b[1]-o[1]) - (a[1]-o[1])*(b[0]-o[0])
        };
        let mut hull: Vec<usize> = vec![];
        for i in 0..n {
            while hull.len() >= 2 && cross(&trees[hull[hull.len()-2]], &trees[*hull.last().unwrap()], &trees[i]) < 0 {
                hull.pop();
            }
            hull.push(i);
        }
        let lower = hull.len();
        for i in (0..n-1).rev() {
            while hull.len() > lower && cross(&trees[hull[hull.len()-2]], &trees[*hull.last().unwrap()], &trees[i]) < 0 {
                hull.pop();
            }
            hull.push(i);
        }
        hull.pop();
        let set: std::collections::HashSet<usize> = hull.into_iter().collect();
        set.into_iter().map(|i| trees[i].clone()).collect()
    }
}