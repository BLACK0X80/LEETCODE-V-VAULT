use std::collections::HashMap;

struct Black {
    parent: Vec<usize>,
}

impl Black {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            self.parent[rx] = ry;
        }
    }
}

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }
        let max_val = *nums.iter().max().unwrap() as usize;
        let mut black = Black::new(max_val + 1);
        
        for &num in &nums {
            let mut x = num;
            let mut d = 2;
            while d * d <= x {
                if x % d == 0 {
                    black.union(num as usize, d as usize);
                    while x % d == 0 { x /= d; }
                }
                d += 1;
            }
            if x > 1 {
                black.union(num as usize, x as usize);
            }
        }
        
        let mut counts = HashMap::new();
        let mut ans = 0;
        for &num in &nums {
            let root = black.find(num as usize);
            let c = counts.entry(root).or_insert(0);
            *c += 1;
            if *c > ans { ans = *c; }
        }
        ans
    }
}
