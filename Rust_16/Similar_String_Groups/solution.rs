impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = find(p, p[x]); }
            p[x]
        }

        let similar = |a: &str, b: &str| -> bool {
            let diffs: Vec<_> = a.bytes().zip(b.bytes()).filter(|(x,y)| x != y).collect();
            diffs.is_empty() || (diffs.len() == 2 && diffs[0].0 == diffs[1].1 && diffs[0].1 == diffs[1].0)
        };

        for i in 0..n {
            for j in i+1..n {
                if similar(&strs[i], &strs[j]) {
                    let pi = find(&mut parent, i);
                    let pj = find(&mut parent, j);
                    if pi != pj { parent[pi] = pj; }
                }
            }
        }

        (0..n).filter(|&i| find(&mut parent, i) == i).count() as i32
    }
}