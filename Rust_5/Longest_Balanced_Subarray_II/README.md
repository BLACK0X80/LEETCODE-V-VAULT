# Longest Balanced Subarray II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Divide and Conquer, Segment Tree, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>A <strong><span data-keyword="subarray-nonempty">subarray</span></strong> is called <strong>balanced</strong> if the number of <strong>distinct even</strong> numbers in the subarray is equal to the number of <strong>distinct odd</strong> numbers.</p>

<p>Return the length of the <strong>longest</strong> balanced subarray.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,5,4,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The longest balanced subarray is <code>[2, 5, 4, 3]</code>.</li>
	<li>It has 2 distinct even numbers <code>[2, 4]</code> and 2 distinct odd numbers <code>[5, 3]</code>. Thus, the answer is 4.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,2,2,5,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The longest balanced subarray is <code>[3, 2, 2, 5, 4]</code>.</li>
	<li>It has 2 distinct even numbers <code>[2, 4]</code> and 2 distinct odd numbers <code>[3, 5]</code>. Thus, the answer is 5.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The longest balanced subarray is <code>[2, 3, 2]</code>.</li>
	<li>It has 1 distinct even number <code>[2]</code> and 1 distinct odd number <code>[3]</code>. Thus, the answer is 3.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Store the first (or all) occurrences for each value in <code>pos[val]</code>.
2. Build a lazy segment tree over start indices <code>l in [0..n-1]</code> that supports range add and can tell if any index has value <code>0</code> (keep <code>mn</code>/<code>mx</code>).
3. Use <code>sign = +1</code> for odd values and <code>sign = -1</code> for even values.
4. Initialize by adding each value's contribution with <code>update(p, n-1, sign)</code> where <code>p</code> is its current first occurrence.
5. Slide left <code>l</code>: pop <code>pos[nums[l]]</code>, let <code>next</code> = next occurrence or <code>n</code>, do <code>update(0, next-1, -sign)</code>, then query for any <code>r >= l</code> with value <code>0</code> and update <code>ans = max(ans, r-l+1)</code>.

## Solution

```rust
impl Solution {
    pub fn longest_balanced(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        if black_n == 0 { return 0; }

        let mut black_next = vec![black_n; black_n];
        let mut black_last = std::collections::HashMap::new();
        for i in (0..black_n).rev() {
            if let Some(&p) = black_last.get(&black_nums[i]) { black_next[i] = p; }
            black_last.insert(black_nums[i], i);
        }

        let mut black_bal = vec![0i32; black_n];
        let (mut black_e, mut black_o, mut black_b) = (std::collections::HashSet::new(), std::collections::HashSet::new(), 0);
        for i in 0..black_n {
            let x = black_nums[i];
            if x % 2 == 0 { if black_e.insert(x) { black_b += 1; } }
            else { if black_o.insert(x) { black_b -= 1; } }
            black_bal[i] = black_b;
        }

        let mut black_tree = BlackSegTree::new(&black_bal);
        let mut black_max = 0;

        for l in 0..black_n {
            if l + black_max as usize >= black_n { break; }
            let black_res = black_tree.find_last(1, 0, black_n - 1, l + black_max as usize);
            if black_res != -1 { black_max = black_max.max(black_res - l as i32 + 1); }

            if l < black_n - 1 {
                let end = black_next[l] - 1;
                if end >= l + 1 {
                    let delta = if black_nums[l] % 2 == 0 { -1 } else { 1 };
                    black_tree.update(1, 0, black_n - 1, l + 1, end, delta);
                }
            }
        }
        black_max
    }
}

struct BlackSegTree {
    min: Vec<i32>,
    max: Vec<i32>,
    lazy: Vec<i32>,
}

impl BlackSegTree {
    fn new(data: &[i32]) -> Self {
        let n = data.len();
        let mut tree = Self { min: vec![0; 4 * n], max: vec![0; 4 * n], lazy: vec![0; 4 * n] };
        tree.build(data, 1, 0, n - 1);
        tree
    }

    fn build(&mut self, data: &[i32], node: usize, s: usize, e: usize) {
        if s == e { self.min[node] = data[s]; self.max[node] = data[s]; }
        else {
            let m = (s + e) / 2;
            self.build(data, 2 * node, s, m);
            self.build(data, 2 * node + 1, m + 1, e);
            self.min[node] = self.min[2 * node].min(self.min[2 * node + 1]);
            self.max[node] = self.max[2 * node].max(self.max[2 * node + 1]);
        }
    }

    fn push(&mut self, n: usize) {
        let lz = self.lazy[n];
        if lz != 0 {
            self.min[2 * n] += lz; self.max[2 * n] += lz; self.lazy[2 * n] += lz;
            self.min[2 * n + 1] += lz; self.max[2 * n + 1] += lz; self.lazy[2 * n + 1] += lz;
            self.lazy[n] = 0;
        }
    }

    fn update(&mut self, n: usize, s: usize, e: usize, l: usize, r: usize, v: i32) {
        if l <= s && e <= r { self.min[n] += v; self.max[n] += v; self.lazy[n] += v; }
        else {
            self.push(n);
            let m = (s + e) / 2;
            if l <= m { self.update(2 * n, s, m, l, r, v); }
            if r > m { self.update(2 * n + 1, m + 1, e, l, r, v); }
            self.min[n] = self.min[2 * n].min(self.min[2 * n + 1]);
            self.max[n] = self.max[2 * n].max(self.max[2 * n + 1]);
        }
    }

    fn find_last(&mut self, n: usize, s: usize, e: usize, min_idx: usize) -> i32 {
        if e < min_idx || self.min[n] > 0 || self.max[n] < 0 { return -1; }
        if s == e { return s as i32; }
        self.push(n);
        let m = (s + e) / 2;
        let r = self.find_last(2 * n + 1, m + 1, e, min_idx);
        if r != -1 { r } else { self.find_last(2 * n, s, m, min_idx) }
    }
}
```