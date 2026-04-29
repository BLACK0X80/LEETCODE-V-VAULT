# Minimum Swaps to Make Strings Equal

**Difficulty:** Medium
**Tags:** Math, String, Greedy

---

## Problem

<p>You are given two strings <code>s1</code> and <code>s2</code> of equal length consisting of letters <code>&quot;x&quot;</code> and <code>&quot;y&quot;</code> <strong>only</strong>. Your task is to make these two strings equal to each other. You can swap any two characters that belong to <strong>different</strong> strings, which means: swap <code>s1[i]</code> and <code>s2[j]</code>.</p>

<p>Return the minimum number of swaps required to make <code>s1</code> and <code>s2</code> equal, or return <code>-1</code> if it is impossible to do so.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;xx&quot;, s2 = &quot;yy&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> Swap s1[0] and s2[1], s1 = &quot;yx&quot;, s2 = &quot;yx&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;xy&quot;, s2 = &quot;yx&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> Swap s1[0] and s2[0], s1 = &quot;yy&quot;, s2 = &quot;xx&quot;.
Swap s1[0] and s2[1], s1 = &quot;xy&quot;, s2 = &quot;xy&quot;.
Note that you cannot swap s1[0] and s1[1] to make s1 equal to &quot;yx&quot;, cause we can only swap chars in different strings.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;xx&quot;, s2 = &quot;xy&quot;
<strong>Output:</strong> -1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s1.length, s2.length &lt;= 1000</code></li>
	<li><code>s1.length == s2.length</code></li>
	<li><code>s1, s2</code> only contain <code>&#39;x&#39;</code> or <code>&#39;y&#39;</code>.</li>
</ul>


## Hints

1. First, ignore all the already matched positions, they don't affect the answer at all. For the unmatched positions, there are three basic cases (already given in the examples):
2. ("xx", "yy") => 1 swap, ("xy", "yx") => 2 swaps
3. So the strategy is, apply case 1 as much as possible, then apply case 2 if the last two unmatched are in this case, or fall into impossible if only one pair of unmatched left. This can be done via a simple math.

## Solution

```rust
impl Solution { pub fn minimum_swap(black_s1: String, black_s2: String) -> i32 { let (mut black_xy, mut black_yx) = (0, 0); black_s1.chars().zip(black_s2.chars()).for_each(|(black_c1, black_c2)| if black_c1 != black_c2 { if black_c1 == 'x' { black_xy += 1 } else { black_yx += 1 } }); if (black_xy + black_yx) % 2 != 0 { -1 } else { black_xy / 2 + black_yx / 2 + (black_xy % 2) * 2 } } }
```