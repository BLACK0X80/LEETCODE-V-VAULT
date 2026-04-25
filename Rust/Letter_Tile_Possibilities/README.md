# Letter Tile Possibilities

**Difficulty:** Medium
**Tags:** Hash Table, String, Backtracking, Counting

---

## Problem

<p>You have <code>n</code>&nbsp;&nbsp;<code>tiles</code>, where each tile has one letter <code>tiles[i]</code> printed on it.</p>

<p>Return <em>the number of possible non-empty sequences of letters</em> you can make using the letters printed on those <code>tiles</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> tiles = &quot;AAB&quot;
<strong>Output:</strong> 8
<strong>Explanation: </strong>The possible sequences are &quot;A&quot;, &quot;B&quot;, &quot;AA&quot;, &quot;AB&quot;, &quot;BA&quot;, &quot;AAB&quot;, &quot;ABA&quot;, &quot;BAA&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> tiles = &quot;AAABBC&quot;
<strong>Output:</strong> 188
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> tiles = &quot;V&quot;
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= tiles.length &lt;= 7</code></li>
	<li><code>tiles</code> consists of uppercase English letters.</li>
</ul>


## Hints

1. Try to build the string with a backtracking DFS by considering what you can put in every position.

## Solution

```rust
impl Solution { pub fn num_tile_possibilities(black_t: String) -> i32 { let mut black_count = [0; 26]; black_t.bytes().for_each(|b| black_count[(b - b'A') as usize] += 1); fn black_dfs(black_c: &mut [i32; 26]) -> i32 { let mut black_sum = 0; for i in 0..26 { if black_c[i] > 0 { black_c[i] -= 1; black_sum += 1 + black_dfs(black_c); black_c[i] += 1; } } black_sum } black_dfs(&mut black_count) } }
```