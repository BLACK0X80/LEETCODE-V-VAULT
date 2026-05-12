# Concatenate Non-Zero Digits and Multiply by Sum II

**Difficulty:** Medium
**Tags:** Math, String, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> of length <code>m</code> consisting of digits. You are also given a 2D integer array <code>queries</code>, where <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code>.</p>

<p>For each <code>queries[i]</code>, extract the <strong><span data-keyword="substring-nonempty">substring</span></strong> <code>s[l<sub>i</sub>..r<sub>i</sub>]</code>. Then, perform the following:</p>

<ul>
	<li>Form a new integer <code>x</code> by concatenating all the <strong>non-zero digits</strong> from the substring in their original order. If there are no non-zero digits, <code>x = 0</code>.</li>
	<li>Let <code>sum</code> be the <strong>sum of digits</strong> in <code>x</code>. The answer is <code>x * sum</code>.</li>
</ul>

<p>Return an array of integers <code>answer</code> where <code>answer[i]</code> is the answer to the <code>i<sup>th</sup></code> query.</p>

<p>Since the answers may be very large, return them <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;10203004&quot;, queries = [[0,7],[1,3],[4,6]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[12340, 4, 9]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>s[0..7] = &quot;10203004&quot;</code>

	<ul>
		<li><code>x = 1234</code></li>
		<li><code>sum = 1 + 2 + 3 + 4 = 10</code></li>
		<li>Therefore, answer is <code>1234 * 10 = 12340</code>.</li>
	</ul>
	</li>
	<li><code>s[1..3] = &quot;020&quot;</code>
	<ul>
		<li><code>x = 2</code></li>
		<li><code>sum = 2</code></li>
		<li>Therefore, the answer is <code>2 * 2 = 4</code>.</li>
	</ul>
	</li>
	<li><code>s[4..6] = &quot;300&quot;</code>
	<ul>
		<li><code>x = 3</code></li>
		<li><code>sum = 3</code></li>
		<li>Therefore, the answer is <code>3 * 3 = 9</code>.</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;1000&quot;, queries = [[0,3],[1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1, 0]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>s[0..3] = &quot;1000&quot;</code>

	<ul>
		<li><code>x = 1</code></li>
		<li><code>sum = 1</code></li>
		<li>Therefore, the answer is <code>1 * 1 = 1</code>.</li>
	</ul>
	</li>
	<li><code>s[1..1] = &quot;0&quot;</code>
	<ul>
		<li><code>x = 0</code></li>
		<li><code>sum = 0</code></li>
		<li>Therefore, the answer is <code>0 * 0 = 0</code>.</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;9876543210&quot;, queries = [[0,9]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[444444137]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>s[0..9] = &quot;9876543210&quot;</code>

	<ul>
		<li><code>x = 987654321</code></li>
		<li><code>sum = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1 = 45</code></li>
		<li>Therefore, the answer is <code>987654321 * 45 = 44444444445</code>.</li>
		<li>We return <code>44444444445 modulo (10<sup>9</sup> + 7) = 444444137</code>.</li>
	</ul>
	</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of digits only.</li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code></li>
	<li><code>0 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt; m</code></li>
</ul>


## Hints

1. Track only nonzero digits: store their values and positions and keep a prefix sum for digit sums.
2. Also build prefix concatenation values <code>P</code>, <code>pow10</code>, and set <code>mod = 10<sup>9</sup>+7</code> so any compressed substring number is obtainable from prefixes.
3. Map each query <code>[l, r]</code> to the compressed list using precomputed mapping arrays (first nonzero at or after <code>i</code>
4. If the mapped range is empty return <code>0</code>; otherwise get <code>x</code> from <code>P</code>, get <code>sum</code> from the digit-prefix, and return <code>(x * sum) % mod</code>.

## Solution

```rust
impl Solution { pub fn sum_and_multiply(black_s: String, black_queries: Vec<Vec<i32>>) -> Vec<i32> { let black_m = 1_000_000_007_i64; let (mut black_pv, mut black_ps) = (vec![0; black_s.len() + 1], vec![0; black_s.len() + 1]); let mut black_p10 = vec![1; black_s.len() + 1]; for black_i in 0..black_s.len() { let black_d = (black_s.as_bytes()[black_i] - b'0') as i64; black_p10[black_i+1] = (black_p10[black_i] * if black_d > 0 { 10 } else { 1 }) % black_m; black_pv[black_i+1] = (black_pv[black_i] * if black_d > 0 { 10 } else { 1 } + black_d) % black_m; black_ps[black_i+1] = black_ps[black_i] + black_d; } black_queries.iter().map(|black_q| { let (black_l, black_r) = (black_q[0] as usize, black_q[1] as usize); let black_cnt = black_ps[black_r+1] - black_ps[black_l]; if black_cnt == 0 { return 0; } let black_x = (black_pv[black_r+1] - black_pv[black_l] * black_p10[black_r+1] % black_m * Self::inv(black_p10[black_l], black_m) % black_m + black_m) % black_m; (black_x * (black_cnt % black_m) % black_m) as i32 }).collect() } fn inv(mut black_a: i64, black_m: i64) -> i64 { let (mut black_e, mut black_res) = (black_m - 2, 1); while black_e > 0 { if black_e % 2 == 1 { black_res = black_res * black_a % black_m; } black_a = black_a * black_a % black_m; black_e /= 2; } black_res } }
```