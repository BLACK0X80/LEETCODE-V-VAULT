# Maximum Sum of Alternating Subsequence With Distance at Least K

**Difficulty:** Hard
**Tags:** 

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and an integer <code>k</code>.</p>

<p>Pick a <strong><span data-keyword="subsequence-sequence">subsequence</span></strong> with indices <code>0 &lt;= i<sub>1</sub> &lt; i<sub>2</sub> &lt; ... &lt; i<sub>m</sub> &lt; n</code> such that:</p>

<ul>
	<li>For every <code>1 &lt;= t &lt; m</code>, <code>i<sub>t+1</sub> - i<sub>t</sub> &gt;= k</code>.</li>
	<li>The selected values form a <strong>strictly alternating</strong> sequence. In other words, either:
	<ul>
		<li><code>nums[i<sub>1</sub>] &lt; nums[i<sub>2</sub>] &gt; nums[i<sub>3</sub>] &lt; ...</code>, or</li>
		<li><code>nums[i<sub>1</sub>] &gt; nums[i<sub>2</sub>] &lt; nums[i<sub>3</sub>] &gt; ...</code></li>
	</ul>
	</li>
</ul>

<p>A <strong>subsequence</strong> of length 1 is also considered <strong>strictly</strong> alternating. The score of a <strong>valid</strong> subsequence is the <strong>sum</strong> of its selected values.</p>

<p>Return an integer denoting the <strong>maximum</strong> possible <strong>score</strong> of a valid subsequence.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,4,2], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<p>An optimal choice is indices <code>[0, 2]</code>, which gives values <code>[5, 2]</code>.</p>

<ul>
	<li>The distance condition holds because <code>2 - 0 = 2 &gt;= k</code>.</li>
	<li>The values are strictly alternating because <code>5 &gt; 2</code>.</li>
</ul>

<p>The score is <code>5 + 2 = 7</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,5,4,2,4], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">14</span></p>

<p><strong>Explanation:</strong></p>

<p>An optimal choice is indices <code>[0, 1, 3, 4]</code>, which gives values <code>[3, 5, 2, 4]</code>.</p>

<ul>
	<li>The distance condition holds because each pair of consecutive chosen indices differs by at least <code>k = 1</code>.</li>
	<li>The values are strictly alternating since <code>3 &lt; 5 &gt; 2 &lt; 4</code>.</li>
</ul>

<p>The score is <code>3 + 5 + 2 + 4 = 14</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>The only valid subsequence is <code>[5]</code>. A subsequence with 1 element is always strictly alternating, so the score is 5.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. Use dynamic programming
2. Let <code>dp[i][val][0/1]</code> represent the maximum score using the first <code>i</code> values, where the last selected value is <code>val</code> and <code>0/1</code> indicates the alternating relation of the last two selected values
3. Use a segment tree to query values greater than or less than <code>val</code> during the DP transitions

## Solution

```rust
impl Solution { pub fn max_alternating_sum(black_n: Vec<i32>, black_k: i32) -> i64 { let (black_s, black_kv, black_mv) = (black_n.len(), black_k as usize, *black_n.iter().max().unwrap() as usize); let (mut black_bu, mut black_bd, mut black_du, mut black_dd, mut black_res) = (vec![0i64; black_mv + 2], vec![0i64; black_mv + 2], vec![0i64; black_s], vec![0i64; black_s], 0i64); for black_i in 0..black_s { if black_i >= black_kv { let (mut black_iu, mut black_id) = (black_n[black_i - black_kv] as usize, black_mv - black_n[black_i - black_kv] as usize + 1); while black_iu <= black_mv { black_bu[black_iu] = black_bu[black_iu].max(black_du[black_i - black_kv]); black_iu += (black_iu as i32 & -(black_iu as i32)) as usize; } while black_id <= black_mv { black_bd[black_id] = black_bd[black_id].max(black_dd[black_i - black_kv]); black_id += (black_id as i32 & -(black_id as i32)) as usize; } } let (mut black_qu, mut black_qd, mut black_mu, mut black_md) = (black_n[black_i] as usize - 1, black_mv - black_n[black_i] as usize, 0i64, 0i64); while black_qu > 0 { black_mu = black_mu.max(black_bu[black_qu]); black_qu -= (black_qu as i32 & -(black_qu as i32)) as usize; } while black_qd > 0 { black_md = black_md.max(black_bd[black_qd]); black_qd -= (black_qd as i32 & -(black_qd as i32)) as usize; } black_du[black_i] = black_n[black_i] as i64 + black_md; black_dd[black_i] = black_n[black_i] as i64 + black_mu; black_res = black_res.max(black_du[black_i]).max(black_dd[black_i]); } black_res } }
```