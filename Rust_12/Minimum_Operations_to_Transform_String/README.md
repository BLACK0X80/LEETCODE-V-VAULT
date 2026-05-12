# Minimum Operations to Transform String

**Difficulty:** Medium
**Tags:** String, Greedy

---

## Problem

<p>You are given a string <code>s</code> consisting only of lowercase English letters.</p>

<p>You can perform the following operation any number of times (including zero):</p>

<ul>
	<li>
	<p>Choose any character <code>c</code> in the string and replace <strong>every</strong> occurrence of <code>c</code> with the <strong>next</strong> lowercase letter in the English alphabet.</p>
	</li>
</ul>

<p>Return the <strong>minimum</strong> number of operations required to transform <code>s</code> into a string consisting of <strong>only</strong> <code>&#39;a&#39;</code> characters.</p>

<p><strong>Note: </strong>Consider the alphabet as circular, thus <code>&#39;a&#39;</code> comes after <code>&#39;z&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;yz&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Change <code>&#39;y&#39;</code> to <code>&#39;z&#39;</code> to get <code>&quot;zz&quot;</code>.</li>
	<li>Change <code>&#39;z&#39;</code> to <code>&#39;a&#39;</code> to get <code>&quot;aa&quot;</code>.</li>
	<li>Thus, the answer is 2.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;a&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The string <code>&quot;a&quot;</code> only consists of <code>&#39;a&#39;</code>​​​​​​​ characters. Thus, the answer is 0.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Each operation shifts every occurrence of a chosen character forward by one in the alphabet (with wrap-around).
2. For any character <code>c</code>, the number of moves required to turn it into <code>'a'</code> is <code>(26 - (ord(c) - ord('a'))) % 26</code>.
3. You can plan operations so characters that need more shifts are advanced first and cause merges that don't increase the total number of moves; therefore the minimum number of moves equals the maximum, over characters appearing in <code>s</code>, of <code>(26 - (ord(c) - ord('a'))) % 26</code>.

## Solution

```rust
impl Solution { pub fn min_operations(black_s: String) -> i32 { let mut black_ans = 0; for black_b in black_s.bytes() { black_ans = black_ans.max(if black_b == b'a' { 0 } else { 26 - (black_b - b'a') as i32 }); } black_ans } }
```