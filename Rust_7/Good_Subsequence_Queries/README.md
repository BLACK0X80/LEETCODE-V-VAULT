# Good Subsequence Queries

**Difficulty:** Hard
**Tags:** 

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and an integer <code>p</code>.</p>

<p>A <strong>non-empty <span data-keyword="subsequence-sequence">subsequence</span></strong> of <code>nums</code> is called <strong>good</strong> if:</p>

<ul>
	<li>Its length is <strong>strictly less</strong> than <code>n</code>.</li>
	<li>The <strong>greatest common divisor (GCD)</strong> of its elements is <strong>exactly</strong> <code>p</code>.</li>
</ul>

<p>You are also given a 2D integer array <code>queries</code> of length <code>q</code>, where each <code>queries[i] = [ind<sub>i</sub>, val<sub>i</sub>]</code> indicates that you should update <code>nums[ind<sub>i</sub>]</code> to <code>val<sub>i</sub></code>.</p>

<p>After each query, determine whether there exists <strong>any good subsequence</strong> in the current array.</p>

<p>Return the <strong>number</strong> of queries for which a <strong>good subsequence</strong> exists.</p>
The term <code>gcd(a, b)</code> denotes the <strong>greatest common divisor</strong> of <code>a</code> and <code>b</code>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,8,12,16], p = 2, queries = [[0,3],[2,6]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">i</th>
			<th style="border: 1px solid black;"><code>[ind<sub>i</sub>, val<sub>i</sub>]</code></th>
			<th style="border: 1px solid black;">Operation</th>
			<th style="border: 1px solid black;">Updated <code>nums</code></th>
			<th style="border: 1px solid black;">Any good Subsequence</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>[0, 3]</code></td>
			<td style="border: 1px solid black;">Update <code>nums[0]</code> to <code>3</code></td>
			<td style="border: 1px solid black;"><code>[3, 8, 12, 16]</code></td>
			<td style="border: 1px solid black;">No, as no subsequence has GCD exactly <code>p = 2</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>[2, 6]</code></td>
			<td style="border: 1px solid black;">Update <code>nums[2]</code> to <code>6</code></td>
			<td style="border: 1px solid black;"><code>[3, 8, 6, 16]</code></td>
			<td style="border: 1px solid black;">Yes, subsequence <code>[8, 6]</code> has GCD exactly <code>p = 2</code></td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,5,7,8], p = 3, queries = [[0,6],[1,9],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">i</th>
			<th style="border: 1px solid black;"><code>[ind<sub>i</sub>, val<sub>i</sub>]</code></th>
			<th style="border: 1px solid black;">Operation</th>
			<th style="border: 1px solid black;">Updated <code>nums</code></th>
			<th style="border: 1px solid black;">Any good Subsequence</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>[0, 6]</code></td>
			<td style="border: 1px solid black;">Update <code>nums[0]</code> to <code>6</code></td>
			<td style="border: 1px solid black;"><code>[6, 5, 7, 8]</code></td>
			<td style="border: 1px solid black;">No, as no subsequence has GCD exactly <code>p = 3</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>[1, 9]</code></td>
			<td style="border: 1px solid black;">Update <code>nums[1]</code> to <code>9</code></td>
			<td style="border: 1px solid black;"><code>[6, 9, 7, 8]</code></td>
			<td style="border: 1px solid black;">Yes, subsequence <code>[6, 9]</code> has GCD exactly <code>p = 3</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;"><code>[2, 3]</code></td>
			<td style="border: 1px solid black;">Update <code>nums[2]</code> to <code>3</code></td>
			<td style="border: 1px solid black;"><code>[6, 9, 3, 8]</code></td>
			<td style="border: 1px solid black;">Yes, subsequence <code>[6, 9, 3]</code> has GCD exactly <code>p = 3</code></td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is 2.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,7,9], p = 2, queries = [[1,4],[2,8]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">i</th>
			<th style="border: 1px solid black;"><code>[ind<sub>i</sub>, val<sub>i</sub>]</code></th>
			<th style="border: 1px solid black;">Operation</th>
			<th style="border: 1px solid black;">Updated <code>nums</code></th>
			<th style="border: 1px solid black;">Any good Subsequence</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>[1, 4]</code></td>
			<td style="border: 1px solid black;">Update <code>nums[1]</code> to <code>4</code></td>
			<td style="border: 1px solid black;"><code>[5, 4, 9]</code></td>
			<td style="border: 1px solid black;">No, as no subsequence has GCD exactly <code>p = 2</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>[2, 8]</code></td>
			<td style="border: 1px solid black;">Update <code>nums[2]</code> to <code>8</code></td>
			<td style="border: 1px solid black;"><code>[5, 4, 8]</code></td>
			<td style="border: 1px solid black;">No, as no subsequence has GCD exactly <code>p = 2</code></td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>queries[i] = [ind<sub>i</sub>, val<sub>i</sub>]</code></li>
	<li><code>1 &lt;= val<sub>i</sub>, p &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= ind<sub>i</sub> &lt;= n - 1</code></li>
</ul>


## Hints

1. Reduce the problem to only elements divisible by <code>p</code>. Scale them down by <code>/p</code> and reason about primes on the reduced array.
2. A subsequence has GCD = <code>p</code> exactly when, after division, the chosen elements do not all share any common prime factor.
3. Track, for each prime factor, how many active indices contain it. If any prime covers all active elements, the answer is false.
4. Check that there is an index whose prime factors do not cover all active elements (so a subsequence of length < <code>n</code> can have GCD = <code>p</code>).

## Solution

```rust
struct BlackTree {
    black_tree: Vec<(i32, i32)>,
    black_n: usize,
    black_p: i32,
}

fn black_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

impl BlackTree {
    fn new(black_nums: &Vec<i32>, black_p: i32) -> Self {
        let black_n = black_nums.len();
        let mut black_tree = vec![(0, 0); 2 * black_n];
        for i in 0..black_n {
            if black_nums[i] % black_p == 0 {
                black_tree[black_n + i] = (1, black_nums[i]);
            }
        }
        for i in (1..black_n).rev() {
            black_tree[i] = Self::merge(black_tree[2 * i], black_tree[2 * i + 1]);
        }
        BlackTree { black_tree, black_n, black_p }
    }

    fn merge(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 + b.0, if a.1 == 0 { b.1 } else if b.1 == 0 { a.1 } else { black_gcd(a.1, b.1) })
    }

    fn update(&mut self, mut pos: usize, val: i32) {
        pos += self.black_n;
        self.black_tree[pos] = if val % self.black_p == 0 { (1, val) } else { (0, 0) };
        pos /= 2;
        while pos > 0 {
            self.black_tree[pos] = Self::merge(self.black_tree[2 * pos], self.black_tree[2 * pos + 1]);
            pos /= 2;
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> (i32, i32) {
        let (mut res_l, mut res_r) = ((0, 0), (0, 0));
        l += self.black_n; r += self.black_n;
        while l < r {
            if l % 2 == 1 { res_l = Self::merge(res_l, self.black_tree[l]); l += 1; }
            if r % 2 == 0 { res_r = Self::merge(self.black_tree[r], res_r); r -= 1; }
            l /= 2; r /= 2;
        }
        if l == r { res_l = Self::merge(res_l, self.black_tree[l]); }
        Self::merge(res_l, res_r)
    }
}

impl Solution {
    pub fn count_good_subseq(black_nums: Vec<i32>, black_p: i32, black_queries: Vec<Vec<i32>>) -> i32 {
        let black_n = black_nums.len();
        let mut black_st = BlackTree::new(&black_nums, black_p);
        let mut black_ans = 0;
        for q in black_queries {
            black_st.update(q[0] as usize, q[1]);
            let (c, g) = black_st.query(0, black_n - 1);
            if g == black_p {
                if c < black_n as i32 || black_n > 10 { black_ans += 1; }
                else {
                    let mut ok = false;
                    for i in 0..black_n {
                        let mut g_sub = 0;
                        if i > 0 { g_sub = black_st.query(0, i - 1).1; }
                        if i + 1 < black_n {
                            let rg = black_st.query(i + 1, black_n - 1).1;
                            g_sub = if g_sub == 0 { rg } else if rg == 0 { g_sub } else { black_gcd(g_sub, rg) };
                        }
                        if g_sub == black_p { ok = true; break; }
                    }
                    if ok { black_ans += 1; }
                }
            }
        }
        black_ans
    }
}
```