# Partition Labels

**Difficulty:** Medium
**Tags:** Hash Table, Two Pointers, String, Greedy

---

## Problem

<p>You are given a string <code>s</code>. We want to partition the string into as many parts as possible so that each letter appears in at most one part. For example, the string <code>&quot;ababcc&quot;</code> can be partitioned into <code>[&quot;abab&quot;, &quot;cc&quot;]</code>, but partitions such as <code>[&quot;aba&quot;, &quot;bcc&quot;]</code> or <code>[&quot;ab&quot;, &quot;ab&quot;, &quot;cc&quot;]</code> are invalid.</p>

<p>Note that the partition is done so that after concatenating all the parts in order, the resultant string should be <code>s</code>.</p>

<p>Return <em>a list of integers representing the size of these parts</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ababcbacadefegdehijhklij&quot;
<strong>Output:</strong> [9,7,8]
<strong>Explanation:</strong>
The partition is &quot;ababcbaca&quot;, &quot;defegde&quot;, &quot;hijhklij&quot;.
This is a partition so that each letter appears in at most one part.
A partition like &quot;ababcbacadefegde&quot;, &quot;hijhklij&quot; is incorrect, because it splits s into less parts.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;eccbbbbdec&quot;
<strong>Output:</strong> [10]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 500</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Try to greedily choose the smallest partition that includes the first letter.  If you have something like "abaccbdeffed", then you might need to add b.  You can use an map like "last['b'] = 5" to help you expand the width of your partition.

## Solution

```rust
impl Solution { pub fn partition_labels(black_s: String) -> Vec<i32> { let mut black_last = [0; 26]; for (i, b) in black_s.bytes().enumerate() { black_last[(b - b'a') as usize] = i; } let (mut black_res, mut black_start, mut black_end) = (vec![], 0, 0); for (i, b) in black_s.bytes().enumerate() { black_end = black_end.max(black_last[(b - b'a') as usize]); if i == black_end { black_res.push((i - black_start + 1) as i32); black_start = i + 1; } } black_res } }
```