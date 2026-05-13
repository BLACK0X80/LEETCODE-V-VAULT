# Partition String Into Minimum Beautiful Substrings

**Difficulty:** Medium
**Tags:** Hash Table, String, Dynamic Programming, Backtracking

---

## Problem

<p>Given a binary string <code>s</code>, partition the string into one or more <strong>substrings</strong> such that each substring is <strong>beautiful</strong>.</p>

<p>A string is <strong>beautiful</strong> if:</p>

<ul>
	<li>It doesn&#39;t contain leading zeros.</li>
	<li>It&#39;s the <strong>binary</strong> representation of a number that is a power of <code>5</code>.</li>
</ul>

<p>Return <em>the <strong>minimum</strong> number of substrings in such partition. </em>If it is impossible to partition the string <code>s</code> into beautiful substrings,&nbsp;return <code>-1</code>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters in a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;1011&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can paritition the given string into [&quot;101&quot;, &quot;1&quot;].
- The string &quot;101&quot; does not contain leading zeros and is the binary representation of integer 5<sup>1</sup> = 5.
- The string &quot;1&quot; does not contain leading zeros and is the binary representation of integer 5<sup>0</sup> = 1.
It can be shown that 2 is the minimum number of beautiful substrings that s can be partitioned into.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;111&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can paritition the given string into [&quot;1&quot;, &quot;1&quot;, &quot;1&quot;].
- The string &quot;1&quot; does not contain leading zeros and is the binary representation of integer 5<sup>0</sup> = 1.
It can be shown that 3 is the minimum number of beautiful substrings that s can be partitioned into.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;0&quot;
<strong>Output:</strong> -1
<strong>Explanation:</strong> We can not partition the given string into beautiful substrings.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 15</code></li>
	<li><code>s[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
</ul>


## Hints

1. To check if number x is a power of 5 or not, we will divide x by 5 while x > 1 and x mod 5 == 0. After iteration if x == 1, then it was a power of 5.
2. Since the constraint of s.length is small, we can use recursion to find all the partitions.

## Solution

```rust
impl Solution { pub fn minimum_beautiful_substrings(s: String) -> i32 { let black_n = s.len(); let mut black_dp = vec![20; black_n + 1]; black_dp[0] = 0; let black_powers: std::collections::HashSet<_> = (0..7).map(|i| format!("{:b}", 5i32.pow(i))).collect(); for i in 1..=black_n { for j in 0..i { if s.as_bytes()[j] != b'0' && black_powers.contains(&s[j..i]) { black_dp[i] = black_dp[i].min(black_dp[j] + 1); } } } if black_dp[black_n] > black_n { -1 } else { black_dp[black_n] as i32 } } }
```