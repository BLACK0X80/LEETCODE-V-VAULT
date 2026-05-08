# Minimum Deletions to Make Alternating Substring

**Difficulty:** Hard
**Tags:** String, Segment Tree

---

## Problem

<p>You are given a string <code>s</code> of length <code>n</code> consisting only of the characters <code>&#39;A&#39;</code> and <code>&#39;B&#39;</code>.</p>

<p>You are also given a 2D integer array <code>queries</code> of length <code>q</code>, where each <code>queries[i]</code> is one of the following:</p>

<ul>
	<li><code>[1, j]</code>: <strong>Flip</strong> the character at index <code>j</code> of <code>s</code> i.e. <code>&#39;A&#39;</code> changes to <code>&#39;B&#39;</code> (and vice versa). This operation mutates <code>s</code> and affects subsequent queries.</li>
	<li><code>[2, l, r]</code>: <strong>Compute</strong> the <strong>minimum</strong> number of character deletions required to make the <strong>substring</strong> <code>s[l..r]</code> <strong>alternating</strong>. This operation does not modify <code>s</code>; the length of <code>s</code> remains <code>n</code>.</li>
</ul>

<p>A <strong><span data-keyword="substring-nonempty">substring</span></strong> is <strong>alternating</strong> if no two <strong>adjacent</strong> characters are <strong>equal</strong>. A substring of length 1 is always alternating.</p>

<p>Return an integer array <code>answer</code>, where <code>answer[i]</code> is the result of the <code>i<sup>th</sup></code> query of type <code>[2, l, r]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;ABA&quot;, queries = [[2,1,2],[1,1],[2,0,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,2]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th align="center" style="border: 1px solid black;"><code><strong>i</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>queries[i]</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>j</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>l</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>r</strong></code></th>
			<th align="center" style="border: 1px solid black;"><strong><code>s</code> before query</strong></th>
			<th align="center" style="border: 1px solid black;"><code><strong>s[l..r]</strong></code></th>
			<th align="center" style="border: 1px solid black;"><strong>Result</strong></th>
			<th align="center" style="border: 1px solid black;"><strong>Answer</strong></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="center" style="border: 1px solid black;">0</td>
			<td align="center" style="border: 1px solid black;">[2, 1, 2]</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;ABA&quot;</code></td>
			<td align="center" style="border: 1px solid black;"><code>&quot;BA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">Already alternating</td>
			<td align="center" style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">[1, 1]</td>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;ABA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">Flip <code>s[1]</code> from <code>&#39;B&#39;</code> to <code>&#39;A&#39;</code></td>
			<td align="center" style="border: 1px solid black;">-</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;">[2, 0, 2]</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">0</td>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;AAA&quot;</code></td>
			<td align="center" style="border: 1px solid black;"><code>&quot;AAA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">Delete any two <code>&#39;A&#39;</code>s to get <code>&quot;A&quot;</code></td>
			<td align="center" style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is <code>[0, 2]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;ABB&quot;, queries = [[2,0,2],[1,2],[2,0,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,0]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th align="center" style="border: 1px solid black;"><code><strong>i</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>queries[i]</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>j</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>l</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>r</strong></code></th>
			<th align="center" style="border: 1px solid black;"><strong><code>s</code> before query</strong></th>
			<th align="center" style="border: 1px solid black;"><code><strong>s[l..r]</strong></code></th>
			<th align="center" style="border: 1px solid black;"><strong>Result</strong></th>
			<th align="center" style="border: 1px solid black;"><strong>Answer</strong></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="center" style="border: 1px solid black;">0</td>
			<td align="center" style="border: 1px solid black;">[2, 0, 2]</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">0</td>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;ABB&quot;</code></td>
			<td align="center" style="border: 1px solid black;"><code>&quot;ABB&quot;</code></td>
			<td align="center" style="border: 1px solid black;">Delete one <code>&#39;B&#39;</code> to get <code>&quot;AB&quot;</code></td>
			<td align="center" style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">[1, 2]</td>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;ABB&quot;</code></td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">Flip <code>s[2]</code> from <code>&#39;B&#39;</code> to <code>&#39;A&#39;</code></td>
			<td align="center" style="border: 1px solid black;">-</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;">[2, 0, 2]</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">0</td>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;ABA&quot;</code></td>
			<td align="center" style="border: 1px solid black;"><code>&quot;ABA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">Already alternating</td>
			<td align="center" style="border: 1px solid black;">0</td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is <code>[1, 0]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;BABA&quot;, queries = [[2,0,3],[1,1],[2,1,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,1]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th align="center" style="border: 1px solid black;"><code><strong>i</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>queries[i]</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>j</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>l</strong></code></th>
			<th align="center" style="border: 1px solid black;"><code><strong>r</strong></code></th>
			<th align="center" style="border: 1px solid black;"><strong><code>s</code> before query</strong></th>
			<th align="center" style="border: 1px solid black;"><code><strong>s[l..r]</strong></code></th>
			<th align="center" style="border: 1px solid black;"><strong>Result</strong></th>
			<th align="center" style="border: 1px solid black;"><strong>Answer</strong></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="center" style="border: 1px solid black;">0</td>
			<td align="center" style="border: 1px solid black;">[2, 0, 3]</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">0</td>
			<td align="center" style="border: 1px solid black;">3</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;BABA&quot;</code></td>
			<td align="center" style="border: 1px solid black;"><code>&quot;BABA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">Already alternating</td>
			<td align="center" style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">[1, 1]</td>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;BABA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">Flip <code>s[1]</code> from <code>&#39;A&#39;</code> to <code>&#39;B&#39;</code></td>
			<td align="center" style="border: 1px solid black;">-</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;">[2, 1, 3]</td>
			<td align="center" style="border: 1px solid black;">-</td>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">3</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;BBBA&quot;</code></td>
			<td align="center" style="border: 1px solid black;"><code>&quot;BBA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">Delete one <code>&#39;B&#39;</code> to get <code>&quot;BA&quot;</code></td>
			<td align="center" style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is <code>[0, 1]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s[i]</code> is either <code>&#39;A&#39;</code> or <code>&#39;B&#39;</code>.</li>
	<li><code>1 &lt;= q == queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i].length == 2</code> or <code>3</code>
	<ul>
		<li><code>queries[i] == [1, j]</code> or,</li>
		<li><code>queries[i] == [2, l, r]</code></li>
		<li><code>0 &lt;= j &lt;= n - 1</code></li>
		<li><code>0 &lt;= l &lt;= r &lt;= n - 1</code></li>
	</ul>
	</li>
</ul>


## Hints

1. Use a Fenwick tree (BIT) over an auxiliary array <code>eq</code>.
2. Define <code>eq[i] = 1</code> if <code>i >= 1</code> and <code>s[i] == s[i - 1]</code>, otherwise <code>eq[i] = 0</code>.
3. For a type-2 query <code>[2, l, r]</code> the answer is <code>sum(eq[l+1..r])</code> (count of equal adjacent pairs in the substring).
4. For a flip <code>[1, j]</code>, recompute and update <code>eq[j]</code> and <code>eq[j + 1]</code>; each flip changes at most two <code>eq</code> values.

## Solution

```rust
struct FenwickTree {
    black_tree: Vec<i32>,
}

impl FenwickTree {
    fn new(black_size: usize) -> Self {
        Self {
            black_tree: vec![0; black_size + 1],
        }
    }

    fn update(&mut self, mut black_i: usize, black_delta: i32) {
        black_i += 1;
        while black_i < self.black_tree.len() {
            self.black_tree[black_i] += black_delta;
            black_i += (black_i as isize & -(black_i as isize)) as usize;
        }
    }

    fn query(&self, mut black_i: usize) -> i32 {
        black_i += 1;
        let mut black_s = 0;
        while black_i > 0 {
            black_s += self.black_tree[black_i];
            black_i -= (black_i as isize & -(black_i as isize)) as usize;
        }
        black_s
    }

    fn query_range(&self, black_l: usize, black_r: usize) -> i32 {
        if black_l > black_r {
            return 0;
        }
        self.query(black_r) - (if black_l > 0 { self.query(black_l - 1) } else { 0 })
    }
}

impl Solution {
    pub fn min_deletions(black_s: String, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_s.len();
        let mut black_list = black_s.into_bytes();
        let mut black_bit = FenwickTree::new(black_n);

        let black_is_bad = |black_idx: usize, black_arr: &[u8]| -> i32 {
            if black_idx < black_n - 1 {
                if black_arr[black_idx] == black_arr[black_idx + 1] { return 1; }
            }
            0
        };

        for black_i in 0..black_n - 1 {
            if black_is_bad(black_i, &black_list) == 1 {
                black_bit.update(black_i, 1);
            }
        }

        let mut black_results = Vec::with_capacity(black_queries.len());
        for black_q in black_queries {
            if black_q[0] == 1 {
                let black_idx = black_q[1] as usize;

                let black_left_before = if black_idx > 0 { black_is_bad(black_idx - 1, &black_list) } else { 0 };
                let black_right_before = black_is_bad(black_idx, &black_list);

                black_list[black_idx] = if black_list[black_idx] == b'A' { b'B' } else { b'A' };

                let black_left_after = if black_idx > 0 { black_is_bad(black_idx - 1, &black_list) } else { 0 };
                let black_right_after = black_is_bad(black_idx, &black_list);

                if black_idx > 0 {
                    black_bit.update(black_idx - 1, black_left_after - black_left_before);
                }
                if black_idx < black_n - 1 {
                    black_bit.update(black_idx, black_right_after - black_right_before);
                }
            } else {
                let black_l = black_q[1] as usize;
                let black_r = black_q[2] as usize;
                if black_l >= black_r {
                    black_results.push(0);
                } else {
                    black_results.push(black_bit.query_range(black_l, black_r - 1));
                }
            }
        }
        black_results
    }
}
```