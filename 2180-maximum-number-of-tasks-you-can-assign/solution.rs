use std::collections::BTreeMap;

impl Solution {
    pub fn max_task_assign(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        tasks.sort(); workers.sort();
        let (n, m) = (tasks.len(), workers.len());

        let check = |mid: usize| -> bool {
            let mut p = pills as usize;
            let mut avail: BTreeMap<i32, usize> = BTreeMap::new();
            for &w in &workers[m-mid..] { *avail.entry(w).or_insert(0) += 1; }
            for &t in tasks[..mid].iter().rev() {
                if let Some((&w, _)) = avail.range(t..).next() {
                    let c = avail.get_mut(&w).unwrap();
                    if *c == 1 { avail.remove(&w); } else { *c -= 1; }
                } else if p > 0 {
                    if let Some((&w, _)) = avail.range(t-strength..).next() {
                        let c = avail.get_mut(&w).unwrap();
                        if *c == 1 { avail.remove(&w); } else { *c -= 1; }
                        p -= 1;
                    } else { return false; }
                } else { return false; }
            }
            true
        };

        let (mut lo, mut hi) = (0, n.min(m));
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if check(mid) { lo = mid; } else { hi = mid - 1; }
        }
        lo as i32
    }
}
