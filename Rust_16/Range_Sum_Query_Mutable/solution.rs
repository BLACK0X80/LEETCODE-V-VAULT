struct NumArray {
    tree: Vec<i32>,
    n: usize,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0; 2 * n];
        for i in 0..n { tree[i + n] = nums[i]; }
        for i in (1..n).rev() { tree[i] = tree[2*i] + tree[2*i+1]; }
        NumArray { tree, n }
    }

    fn update(&mut self, mut index: i32, val: i32) {
        let mut i = index as usize + self.n;
        self.tree[i] = val;
        i >>= 1;
        while i >= 1 { self.tree[i] = self.tree[2*i] + self.tree[2*i+1]; i >>= 1; }
    }

    fn sum_range(&self, mut left: i32, mut right: i32) -> i32 {
        let (mut l, mut r) = (left as usize + self.n, right as usize + self.n + 1);
        let mut s = 0;
        while l < r {
            if l & 1 == 1 { s += self.tree[l]; l += 1; }
            if r & 1 == 1 { r -= 1; s += self.tree[r]; }
            l >>= 1; r >>= 1;
        }
        s
    }
}