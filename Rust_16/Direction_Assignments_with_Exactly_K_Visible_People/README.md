# Direction Assignments with Exactly K Visible People

**Difficulty:** Medium
**Tags:** Math, Combinatorics

---

## Problem

<p>You are given three integers <code>n</code>, <code>pos</code>, and <code>k</code>.</p>

<p>There are <code>n</code> people standing in a line indexed from 0 to <code>n - 1</code>. Each person <strong>independently</strong> chooses a direction:</p>

<ul>
	<li><code>&#39;L&#39;</code>: <strong>visible</strong> only to people on their <strong>right</strong></li>
	<li><code>&#39;R&#39;</code>: <strong>visible</strong> only to people on their <strong>left</strong></li>
</ul>
A person at index <code>pos</code> sees others as follows:

<ul>
	<li>A person <code>i &lt; pos</code> is visible if and only if they choose <code>&#39;L&#39;</code>.</li>
	<li>A person <code>i &gt; pos</code> is visible if and only if they choose <code>&#39;R&#39;</code>.</li>
</ul>

<p>Return the number of possible direction assignments such that the person at index <code>pos</code> sees <strong>exactly</strong> <code>k</code> people.</p>

<p>Since the answer may be large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, pos = 1, k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<ul>
	<li>Index 0 is to the left of <code>pos = 1</code>, and index 2 is to the right of <code>pos = 1</code>.</li>
	<li>To see <code>k = 0</code> people, index 0 must choose <code>&#39;R&#39;</code> and index 2 must choose <code>&#39;L&#39;</code>, keeping both invisible.</li>
	<li>The person at index 1 can choose <code>&#39;L&#39;</code> or <code>&#39;R&#39;</code> since it does not affect the count. Thus, the answer is 2.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, pos = 2, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Index 0 and index 1 are left of <code>pos = 2</code>, and there is no index to the right.</li>
	<li>To see <code>k = 1</code> person, exactly one of index 0 or index 1 must choose <code>&#39;L&#39;</code>, and the other must choose <code>&#39;R&#39;</code>.</li>
	<li>There are 2 ways to choose which index is visible from the left.</li>
	<li>The person at index 2 can choose <code>&#39;L&#39;</code> or <code>&#39;R&#39;</code> since it does not affect the count. Thus, the answer is <code>2 + 2 = 4</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1, pos = 0, k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There are no indices to the left or right of <code>pos = 0</code>.</li>
	<li>To see <code>k = 0</code> people, no additional condition is required.</li>
	<li>The person at index 0 can choose <code>&#39;L&#39;</code> or <code>&#39;R&#39;</code>. Thus, the answer is 2.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= pos, k &lt;= n - 1</code></li>
</ul>


## Hints

1. Use combinatorics
2. Count the visible people on the left and right independently. Let <code>a = pos</code> and <code>b = n - pos - 1</code>.
3. To see exactly <code>k</code> people, choose <code>i</code> visible people from the left side and <code>k - i</code> from the right side.
4. Each chosen left person must pick <code>L</code>, each chosen right person must pick <code>R</code>, and every other person can pick either direction.
5. Sum over all valid <code>i</code>, multiplying the two binomial choices and the free choices for the remaining people.

## Solution

```rust
impl Solution { pub fn count_visible_people(black_n: i32, black_pos: i32, black_k: i32) -> i32 { let (black_m, mut black_ans, mut black_inv) = (1_000_000_007_i64, 1_i64, vec![1_i64; black_k as usize + 1]); for black_i in 1..=black_k as usize { black_ans = black_ans * (black_n as i64 - black_i as i64) % black_m; let (mut black_e, mut black_b, mut black_res) = (black_m - 2, black_i as i64, 1_i64); while black_e > 0 { if black_e % 2 == 1 { black_res = black_res * black_b % black_m; } black_b = black_b * black_b % black_m; black_e /= 2; } black_ans = black_ans * black_res % black_m; } (black_ans * 2 % black_m) as i32 } }
```