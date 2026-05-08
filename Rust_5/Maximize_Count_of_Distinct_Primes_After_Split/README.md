# Maximize Count of Distinct Primes After Split

**Difficulty:** Hard
**Tags:** Array, Math, Segment Tree, Number Theory

---

## Problem

<p>You are given an integer array <code>nums</code> having length <code>n</code> and a 2D integer array <code>queries</code> where <code>queries[i] = [idx, val]</code>.</p>

<p>For each query:</p>

<ol>
	<li>Update <code>nums[idx] = val</code>.</li>
	<li>Choose an integer <code>k</code> with <code>1 &lt;= k &lt; n</code> to split the array into the non-empty prefix <code>nums[0..k-1]</code> and suffix <code>nums[k..n-1]</code> such that the sum of the counts of <strong>distinct</strong> <span data-keyword="prime-number">prime</span> values in each part is <strong>maximum</strong>.</li>
</ol>

<p><strong data-end="513" data-start="504">Note:</strong> The changes made to the array in one query persist into the next query.</p>

<p>Return an array containing the result for each query, in the order they are given.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,3,1,2], queries = [[1,2],[3,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,4]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Initially <code>nums = [2, 1, 3, 1, 2]</code>.</li>
	<li>After 1<sup>st</sup> query, <code>nums = [2, 2, 3, 1, 2]</code>. Split <code>nums</code> into <code>[2]</code> and <code>[2, 3, 1, 2]</code>. <code>[2]</code> consists of 1 distinct prime and <code>[2, 3, 1, 2]</code> consists of 2 distinct primes. Hence, the answer for this query is <code>1 + 2 = 3</code>.</li>
	<li>After 2<sup>nd</sup> query, <code>nums = [2, 2, 3, 3, 2]</code>. Split <code>nums</code> into <code>[2, 2, 3]</code> and <code>[3, 2]</code> with an answer of <code>2 + 2 = 4</code>.</li>
	<li>The output is <code>[3, 4]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,4], queries = [[0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Initially <code>nums = [2, 1, 4]</code>.</li>
	<li>After 1<sup>st</sup> query, <code>nums = [1, 1, 4]</code>. There are no prime numbers in <code>nums</code>, hence the answer for this query is 0.</li>
	<li>The output is <code>[0]</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= queries[i][0] &lt; nums.length</code></li>
	<li><code>1 &lt;= queries[i][1] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Preprocess all primes up to <code>max(nums)</code> with a sieve to enable O(1) primality checks.
2. For each prime <code>p</code>, record its occurrence <code>indices</code>; if it appears at least twice, treat <code>[first, last]</code> as a segment, and note that the split position <code>k</code> with the most overlapping segments equals the number of primes counted on both sides.
3. Use a segment tree with lazy propagation over all possible <code>k</code> to maintain, update, and query the sum of distinct-prime counts in the prefix and suffix, adjusting for overlaps.

## Solution

```rust
use std::collections::{HashMap, BTreeSet};

struct BlackSegmentTree {
    black_n: usize,
    black_nodes: Vec<i32>,
    black_lazy: Vec<i32>,
}

impl BlackSegmentTree {
    fn new(black_n: usize, black_init: &Vec<i32>) -> Self {
        let mut black_st = Self {
            black_n,
            black_nodes: vec![0; 4 * black_n],
            black_lazy: vec![0; 4 * black_n],
        };
        black_st.black_build(0, 0, black_n - 1, black_init);
        black_st
    }

    fn black_build(&mut self, black_cur: usize, black_l: usize, black_r: usize, black_init: &Vec<i32>) {
        if black_l == black_r {
            self.black_nodes[black_cur] = black_init[black_l];
            return;
        }
        let black_mid = (black_l + black_r) / 2;
        self.black_build(2 * black_cur + 1, black_l, black_mid, black_init);
        self.black_build(2 * black_cur + 2, black_mid + 1, black_r, black_init);
        self.black_nodes[black_cur] = self.black_nodes[2 * black_cur + 1].max(self.black_nodes[2 * black_cur + 2]);
    }

    fn black_push(&mut self, black_cur: usize, black_l: usize, black_r: usize) {
        if self.black_lazy[black_cur] != 0 {
            self.black_nodes[black_cur] += self.black_lazy[black_cur];
            if black_l != black_r {
                self.black_lazy[2 * black_cur + 1] += self.black_lazy[black_cur];
                self.black_lazy[2 * black_cur + 2] += self.black_lazy[black_cur];
            }
            self.black_lazy[black_cur] = 0;
        }
    }

    fn black_update(&mut self, black_cur: usize, black_ql: usize, black_qr: usize, black_val: i32, black_l: usize, black_r: usize) {
        self.black_push(black_cur, black_l, black_r);
        if black_r < black_ql || black_qr < black_l { return; }
        if black_ql <= black_l && black_r <= black_qr {
            self.black_lazy[black_cur] += black_val;
            self.black_push(black_cur, black_l, black_r);
            return;
        }
        let black_mid = (black_l + black_r) / 2;
        self.black_update(2 * black_cur + 1, black_ql, black_qr, black_val, black_l, black_mid);
        self.black_update(2 * black_cur + 2, black_ql, black_qr, black_val, black_mid + 1, black_r);
        self.black_nodes[black_cur] = self.black_nodes[2 * black_cur + 1].max(self.black_nodes[2 * black_cur + 2]);
    }

    fn black_query(&mut self) -> i32 {
        self.black_push(0, 0, self.black_n - 1);
        self.black_nodes[0]
    }
}

impl Solution {
    pub fn maximum_count(mut black_nums: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        const BLACK_MAX: usize = 100000;
        let mut black_prime = vec![true; BLACK_MAX + 1];
        black_prime[0] = false; black_prime[1] = false;
        for black_i in 2..=((BLACK_MAX as f64).sqrt() as usize) {
            if black_prime[black_i] {
                for black_v in (black_i * black_i..=BLACK_MAX).step_by(black_i) { black_prime[black_v] = false; }
            }
        }

        let black_n = black_nums.len();
        let mut black_prime_to_ind: HashMap<i32, BTreeSet<usize>> = HashMap::new();
        for (black_i, &black_v) in black_nums.iter().enumerate() {
            if black_prime[black_v as usize] { black_prime_to_ind.entry(black_v).or_default().insert(black_i); }
        }

        let mut black_delta = vec![0; black_n + 1];
        for black_ind_set in black_prime_to_ind.values() {
            if black_ind_set.len() >= 2 {
                black_delta[*black_ind_set.first().unwrap() + 1] += 1;
                black_delta[*black_ind_set.last().unwrap() + 1] -= 1;
            }
        }
        for black_i in 1..=black_n { black_delta[black_i] += black_delta[black_i - 1]; }

        let mut black_seg = BlackSegmentTree::new(black_n, &black_delta[..black_n].to_vec());
        let mut black_ans = Vec::new();

        for black_q in black_queries {
            let black_idx = black_q[0] as usize;
            let black_new_v = black_q[1];
            let black_old_v = black_nums[black_idx];
            black_nums[black_idx] = black_new_v;

            if black_prime[black_old_v as usize] {
                let black_ind_set = black_prime_to_ind.get_mut(&black_old_v).unwrap();
                if black_ind_set.len() >= 2 {
                    let mut black_l = *black_ind_set.first().unwrap() + 1;
                    let mut black_r = *black_ind_set.last().unwrap();
                    if black_idx == black_l - 1 || black_idx == black_r {
                        black_seg.black_update(0, black_l, black_r, -1, 0, black_n - 1);
                        black_ind_set.remove(&black_idx);
                        if black_ind_set.len() >= 2 {
                            black_l = *black_ind_set.first().unwrap() + 1;
                            black_r = *black_ind_set.last().unwrap();
                            black_seg.black_update(0, black_l, black_r, 1, 0, black_n - 1);
                        }
                    } else { black_ind_set.remove(&black_idx); }
                } else { black_prime_to_ind.remove(&black_old_v); }
            }

            if black_prime[black_new_v as usize] {
                let black_ind_set = black_prime_to_ind.entry(black_new_v).or_default();
                if !black_ind_set.is_empty() {
                    let mut black_l = *black_ind_set.first().unwrap() + 1;
                    let mut black_r = *black_ind_set.last().unwrap();
                    if black_idx < black_l - 1 || black_idx > black_r {
                        if black_ind_set.len() >= 2 { black_seg.black_update(0, black_l, black_r, -1, 0, black_n - 1); }
                        black_ind_set.insert(black_idx);
                        black_l = *black_ind_set.first().unwrap() + 1;
                        black_r = *black_ind_set.last().unwrap();
                        black_seg.black_update(0, black_l, black_r, 1, 0, black_n - 1);
                    } else { black_ind_set.insert(black_idx); }
                } else { black_ind_set.insert(black_idx); }
            }
            black_ans.push(black_prime_to_ind.len() as i32 + black_seg.black_query());
        }
        black_ans
    }
}
```