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
    pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        let mut black = Black::new(n as usize);
        let mut res = Vec::new();
        
        for req in &requests {
            let u = req[0] as usize;
            let v = req[1] as usize;
            let ru = black.find(u);
            let rv = black.find(v);
            
            if ru == rv {
                res.push(true);
                continue;
            }
            
            let mut possible = true;
            for rest in &restrictions {
                let x = rest[0] as usize;
                let y = rest[1] as usize;
                let rx = black.find(x);
                let ry = black.find(y);
                
                if (rx == ru && ry == rv) || (rx == rv && ry == ru) {
                    possible = false;
                    break;
                }
            }
            
            if possible {
                black.union(u, v);
                res.push(true);
            } else {
                res.push(false);
            }
        }
        res
    }
}