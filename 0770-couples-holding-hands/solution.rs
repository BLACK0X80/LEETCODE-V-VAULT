impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len() / 2;
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = find(p, p[x]); }
            p[x]
        }

        for i in (0..row.len()).step_by(2) {
            let a = find(&mut parent, row[i] as usize / 2);
            let b = find(&mut parent, row[i+1] as usize / 2);
            if a != b { parent[a] = b; }
        }

        let mut components = 0;
        for i in 0..n {
            if find(&mut parent, i) == i { components += 1; }
        }
        (n - components) as i32
    }
}
