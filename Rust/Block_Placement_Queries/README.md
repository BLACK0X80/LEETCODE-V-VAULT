# Block Placement Queries

**Difficulty:** Hard
**Tags:** Array, Binary Search, Binary Indexed Tree, Segment Tree

---

## Problem

<p>There exists an infinite number line, with its origin at 0 and extending towards the <strong>positive</strong> x-axis.</p>

<p>You are given a 2D array <code>queries</code>, which contains two types of queries:</p>

<ol>
	<li>For a query of type 1, <code>queries[i] = [1, x]</code>. Build an obstacle at distance <code>x</code> from the origin. It is guaranteed that there is <strong>no</strong> obstacle at distance <code>x</code> when the query is asked.</li>
	<li>For a query of type 2, <code>queries[i] = [2, x, sz]</code>. Check if it is possible to place a block of size <code>sz</code> <em>anywhere</em> in the range <code>[0, x]</code> on the line, such that the block <strong>entirely</strong> lies in the range <code>[0, x]</code>. A block <strong>cannot </strong>be placed if it intersects with any obstacle, but it may touch it. Note that you do<strong> not</strong> actually place the block. Queries are separate.</li>
</ol>

<p>Return a boolean array <code>results</code>, where <code>results[i]</code> is <code>true</code> if you can place the block specified in the <code>i<sup>th</sup></code> query of type 2, and <code>false</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">queries = [[1,2],[2,3,3],[2,3,1],[2,2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[false,true,true]</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/04/22/example0block.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 309px; height: 129px;" /></strong></p>

<p>For query 0, place an obstacle at <code>x = 2</code>. A block of size at most 2 can be placed before <code>x = 3</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">queries = </span>[[1,7],[2,7,6],[1,2],[2,7,5],[2,7,6]]<!-- notionvc: 4a471445-5af1-4d72-b11b-94d351a2c8e9 --></p>

<p><strong>Output:</strong> [true,true,false]</p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/04/22/example1block.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 310px; height: 130px;" /></strong></p>

<ul>
	<li>Place an obstacle at <code>x = 7</code> for query 0. A block of size at most 7 can be placed before <code>x = 7</code>.</li>
	<li>Place an obstacle at <code>x = 2</code> for query 2. Now, a block of size at most 5 can be placed before <code>x = 7</code>, and a block of size at most 2 before <code>x = 2</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= queries.length &lt;= 15 * 10<sup>4</sup></code></li>
	<li><code>2 &lt;= queries[i].length &lt;= 3</code></li>
	<li><code>1 &lt;= queries[i][0] &lt;= 2</code></li>
	<li><code>1 &lt;= x, sz &lt;= min(5 * 10<sup>4</sup>, 3 * queries.length)</code></li>
	<li>The input is generated such that for queries of type 1, no obstacle exists at distance <code>x</code> when the query is asked.</li>
	<li>The input is generated such that there is at least one query of type 2.</li>
</ul>


## Hints

1. Let <code>d[x]</code> be the distance of the next obstacle after <code>x</code>.
2. For each query of type 2, we just need to check if <code>max(d[0], d[1], d[2], …d[x - sz]) > sz</code>.
3. Use segment tree to maintain <code>d[x]</code>.

## Solution

```rust
impl Solution {
    pub fn get_results(black_qs: Vec<Vec<i32>>) -> Vec<bool> {
        let (mut black_b, mut black_s) = (vec![0; 50005], std::collections::BTreeSet::from([0, 50001]));
        for black_q in &black_qs { if black_q[0] == 1 { black_s.insert(black_q[1]); } }
        let (black_o, mut black_u) = (black_s.iter().cloned().collect::<Vec<_>>(), |mut black_i: usize, black_v: i32, black_t: &mut Vec<i32>| { while black_i < 50005 { black_t[black_i] = black_t[black_i].max(black_v); black_i += black_i & black_i.wrapping_neg(); } });
        for black_i in 1..black_o.len() { black_u(black_o[black_i] as usize, black_o[black_i] - black_o[black_i-1], &mut black_b); }
        black_qs.into_iter().rev().filter_map(|black_q| if black_q[0] == 1 {
            black_s.remove(&black_q[1]); let black_p = *black_s.range(..black_q[1]).next_back().unwrap();
            if let Some(&black_n) = black_s.range(black_q[1]..).next() { black_u(black_n as usize, black_n - black_p, &mut black_b); } None
        } else {
            let (black_x, mut black_i, mut black_m) = (black_q[1], black_q[1] as usize, 0);
            while black_i > 0 { black_m = black_m.max(black_b[black_i]); black_i &= black_i - 1; }
            Some(black_m.max(black_x - *black_s.range(..=black_x).next_back().unwrap()) >= black_q[2])
        }).collect::<Vec<_>>().into_iter().rev().collect()
    }
}
```