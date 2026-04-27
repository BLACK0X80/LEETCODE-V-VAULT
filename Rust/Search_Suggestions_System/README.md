# Search Suggestions System

**Difficulty:** Medium
**Tags:** Array, String, Binary Search, Trie, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given an array of strings <code>products</code> and a string <code>searchWord</code>.</p>

<p>Design a system that suggests at most three product names from <code>products</code> after each character of <code>searchWord</code> is typed. Suggested products should have common prefix with <code>searchWord</code>. If there are more than three products with a common prefix return the three lexicographically minimums products.</p>

<p>Return <em>a list of lists of the suggested products after each character of </em><code>searchWord</code><em> is typed</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> products = [&quot;mobile&quot;,&quot;mouse&quot;,&quot;moneypot&quot;,&quot;monitor&quot;,&quot;mousepad&quot;], searchWord = &quot;mouse&quot;
<strong>Output:</strong> [[&quot;mobile&quot;,&quot;moneypot&quot;,&quot;monitor&quot;],[&quot;mobile&quot;,&quot;moneypot&quot;,&quot;monitor&quot;],[&quot;mouse&quot;,&quot;mousepad&quot;],[&quot;mouse&quot;,&quot;mousepad&quot;],[&quot;mouse&quot;,&quot;mousepad&quot;]]
<strong>Explanation:</strong> products sorted lexicographically = [&quot;mobile&quot;,&quot;moneypot&quot;,&quot;monitor&quot;,&quot;mouse&quot;,&quot;mousepad&quot;].
After typing m and mo all products match and we show user [&quot;mobile&quot;,&quot;moneypot&quot;,&quot;monitor&quot;].
After typing mou, mous and mouse the system suggests [&quot;mouse&quot;,&quot;mousepad&quot;].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> products = [&quot;havana&quot;], searchWord = &quot;havana&quot;
<strong>Output:</strong> [[&quot;havana&quot;],[&quot;havana&quot;],[&quot;havana&quot;],[&quot;havana&quot;],[&quot;havana&quot;],[&quot;havana&quot;]]
<strong>Explanation:</strong> The only word &quot;havana&quot; will be always suggested while typing the search word.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= products.length &lt;= 1000</code></li>
	<li><code>1 &lt;= products[i].length &lt;= 3000</code></li>
	<li><code>1 &lt;= sum(products[i].length) &lt;= 2 * 10<sup>4</sup></code></li>
	<li>All the strings of <code>products</code> are <strong>unique</strong>.</li>
	<li><code>products[i]</code> consists of lowercase English letters.</li>
	<li><code>1 &lt;= searchWord.length &lt;= 1000</code></li>
	<li><code>searchWord</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Brute force is a good choice because length of the string is ≤ 1000.
2. Binary search the answer.
3. Use Trie data structure to store the best three matching. Traverse the Trie.

## Solution

```rust
impl Solution { pub fn suggested_products(mut black_p: Vec<String>, black_s: String) -> Vec<Vec<String>> { black_p.sort_unstable(); let (mut black_res, mut black_prefix) = (vec![], String::new()); for black_c in black_s.chars() { black_prefix.push(black_c); let black_idx = black_p.partition_point(|black_x| black_x < &black_prefix); black_res.push(black_p.iter().skip(black_idx).take(3).filter(|black_x| black_x.starts_with(&black_prefix)).cloned().collect()); } black_res } }
```