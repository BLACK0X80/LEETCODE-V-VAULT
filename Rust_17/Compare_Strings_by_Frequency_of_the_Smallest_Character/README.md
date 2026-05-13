# Compare Strings by Frequency of the Smallest Character

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Binary Search, Sorting

---

## Problem

<p>Let the function <code>f(s)</code> be the <strong>frequency of the lexicographically smallest character</strong> in a non-empty string <code>s</code>. For example, if <code>s = &quot;dcce&quot;</code> then <code>f(s) = 2</code> because the lexicographically smallest character is <code>&#39;c&#39;</code>, which has a frequency of 2.</p>

<p>You are given an array of strings <code>words</code> and another array of query strings <code>queries</code>. For each query <code>queries[i]</code>, count the <strong>number of words</strong> in <code>words</code> such that <code>f(queries[i])</code> &lt; <code>f(W)</code> for each <code>W</code> in <code>words</code>.</p>

<p>Return <em>an integer array </em><code>answer</code><em>, where each </em><code>answer[i]</code><em> is the answer to the </em><code>i<sup>th</sup></code><em> query</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> queries = [&quot;cbd&quot;], words = [&quot;zaaaz&quot;]
<strong>Output:</strong> [1]
<strong>Explanation:</strong> On the first query we have f(&quot;cbd&quot;) = 1, f(&quot;zaaaz&quot;) = 3 so f(&quot;cbd&quot;) &lt; f(&quot;zaaaz&quot;).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> queries = [&quot;bbb&quot;,&quot;cc&quot;], words = [&quot;a&quot;,&quot;aa&quot;,&quot;aaa&quot;,&quot;aaaa&quot;]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> On the first query only f(&quot;bbb&quot;) &lt; f(&quot;aaaa&quot;). On the second query both f(&quot;aaa&quot;) and f(&quot;aaaa&quot;) are both &gt; f(&quot;cc&quot;).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= queries.length &lt;= 2000</code></li>
	<li><code>1 &lt;= words.length &lt;= 2000</code></li>
	<li><code>1 &lt;= queries[i].length, words[i].length &lt;= 10</code></li>
	<li><code>queries[i][j]</code>, <code>words[i][j]</code> consist of lowercase English letters.</li>
</ul>


## Hints

1. For each string from words calculate the leading count and store it in an array, then sort the integer array.
2. For each string from queries calculate the leading count "p" and in base of the sorted array calculated on the step 1 do a binary search to count the number of items greater than "p".

## Solution

```rust
impl Solution { pub fn num_smaller_by_frequency(black_q: Vec<String>, black_w: Vec<String>) -> Vec<i32> { fn black_f(black_s: &String) -> i32 { let black_min = black_s.chars().min().unwrap(); black_s.chars().filter(|&black_c| black_c == black_min).count() as i32 } let mut black_wf: Vec<i32> = black_w.iter().map(black_f).collect(); black_wf.sort_unstable(); black_q.iter().map(|black_s| { let black_val = black_f(black_s); (black_wf.len() - black_wf.partition_point(|&black_x| black_x <= black_val)) as i32 }).collect() } }
```