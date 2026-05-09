# Integers With Multiple Sum of Two Cubes

**Difficulty:** Medium
**Tags:** Hash Table, Sorting, Counting, Enumeration

---

## Problem

<p>You are given an integer <code>n</code>.</p>

<p>An integer <code>x</code> is considered <strong>good</strong> if there exist <strong>at least</strong> two <strong>distinct</strong> pairs <code>(a, b)</code> such that:</p>

<ul>
	<li><code>a</code> and <code>b</code> are positive integers.</li>
	<li><code>a &lt;= b</code></li>
	<li><code>x = a<sup>3</sup> + b<sup>3</sup></code></li>
</ul>

<p>Return an array containing all good integers <strong>less than or equal to</strong> <code>n</code>, sorted in ascending order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4104</span></p>

<p><strong>Output:</strong> <span class="example-io">[1729,4104]</span></p>

<p><strong>Explanation:</strong></p>

<p>Among integers less than or equal to 4104, the good integers are:</p>

<ul>
	<li>1729: <code>1<sup>3</sup> + 12<sup>3</sup> = 1729</code> and <code>9<sup>3</sup> + 10<sup>3</sup> = 1729</code>.</li>
	<li>4104: <code>2<sup>3</sup> + 16<sup>3</sup> = 4104</code> and <code>9<sup>3</sup> + 15<sup>3</sup> = 4104</code>.</li>
</ul>

<p>Thus, the answer is <code>[1729, 4104]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 578</span></p>

<p><strong>Output:</strong> <span class="example-io">[]</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no good integers less than or equal to 578, so the answer is an empty array.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Iterate over all pairs <code>(a, b)</code> with <code>a <= b</code> and <code>a<sup>3</sup> + b<sup>3</sup> <= n</code>
2. Use a map to count how many times each value of <code>a<sup>3</sup> + b<sup>3</sup></code> appears
3. A value is good if it can be formed by at least two distinct pairs
4. Collect all such values and return them in sorted order

## Solution

```rust
impl Solution { pub fn find_good_integers(n: i32) -> Vec<i32> { let mut black_cnt = std::collections::HashMap::new(); let mut black_a = 1; while black_a * black_a * black_a <= n { let mut black_b = black_a; while black_a * black_a * black_a + black_b * black_b * black_b <= n { *black_cnt.entry(black_a * black_a * black_a + black_b * black_b * black_b).or_insert(0) += 1; black_b += 1; } black_a += 1; } let mut black_res: Vec<i32> = black_cnt.into_iter().filter(|&(_, black_v)| black_v >= 2).map(|(black_k, _)| black_k).collect(); black_res.sort_unstable(); black_res } }
```