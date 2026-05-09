# Minimum Substring Partition of Equal Character Frequency

**Difficulty:** Medium
**Tags:** Hash Table, String, Dynamic Programming, Counting

---

## Problem

<p>Given a string <code>s</code>, you need to partition it into one or more <strong>balanced</strong> <span data-keyword="substring">substrings</span>. For example, if <code>s == &quot;ababcc&quot;</code> then <code>(&quot;abab&quot;, &quot;c&quot;, &quot;c&quot;)</code>, <code>(&quot;ab&quot;, &quot;abc&quot;, &quot;c&quot;)</code>, and <code>(&quot;ababcc&quot;)</code> are all valid partitions, but <code>(&quot;a&quot;, <strong>&quot;bab&quot;</strong>, &quot;cc&quot;)</code>, <code>(<strong>&quot;aba&quot;</strong>, &quot;bc&quot;, &quot;c&quot;)</code>, and <code>(&quot;ab&quot;, <strong>&quot;abcc&quot;</strong>)</code> are not. The unbalanced substrings are bolded.</p>

<p>Return the <strong>minimum</strong> number of substrings that you can partition <code>s</code> into.</p>

<p><strong>Note:</strong> A <strong>balanced</strong> string is a string where each character in the string occurs the same number of times.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;fabccddg&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>We can partition the string <code>s</code> into 3 substrings in one of the following ways: <code>(&quot;fab, &quot;ccdd&quot;, &quot;g&quot;)</code>, or <code>(&quot;fabc&quot;, &quot;cd&quot;, &quot;dg&quot;)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abababaccddb&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We can partition the string <code>s</code> into 2 substrings like so: <code>(&quot;abab&quot;, &quot;abaccddb&quot;)</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
	<li><code>s</code> consists only of English lowercase letters.</li>
</ul>


## Hints

1. Let <code>dp[i]</code> be the minimum number of partitions for the prefix ending at index <code>i + 1</code>.
2. <code>dp[i]</code> can be calculated as the <code>min(dp[j])</code> over all <code>j</code> such that <code>j < i</code> and <code>word[j+1…i]</code> is valid.

## Solution

```rust
impl Solution { pub fn minimum_substrings_in_partition(s: String) -> i32 { let (n, s_bytes) = (s.len(), s.as_bytes()); let mut black_dp = vec![1001; n + 1]; black_dp[0] = 0; for i in 1..=n { let mut counts = [0; 26]; for j in (0..i).rev() { counts[(s_bytes[j] - b'a') as usize] += 1; let (mut max_f, mut min_f, mut distinct) = (0, 1001, 0); for &f in &counts { if f > 0 { max_f = max_f.max(f); min_f = min_f.min(f); distinct += 1; } } if max_f == min_f { black_dp[i] = black_dp[i].min(black_dp[j] + 1); } } } black_dp[n] } }
```