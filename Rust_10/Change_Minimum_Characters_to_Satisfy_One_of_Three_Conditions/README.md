# Change Minimum Characters to Satisfy One of Three Conditions

**Difficulty:** Medium
**Tags:** Hash Table, String, Counting, Prefix Sum

---

## Problem

<p>You are given two strings <code>a</code> and <code>b</code> that consist of lowercase letters. In one operation, you can change any character in <code>a</code> or <code>b</code> to <strong>any lowercase letter</strong>.</p>

<p>Your goal is to satisfy <strong>one</strong> of the following three conditions:</p>

<ul>
	<li><strong>Every</strong> letter in <code>a</code> is <strong>strictly less</strong> than <strong>every</strong> letter in <code>b</code> in the alphabet.</li>
	<li><strong>Every</strong> letter in <code>b</code> is <strong>strictly less</strong> than <strong>every</strong> letter in <code>a</code> in the alphabet.</li>
	<li><strong>Both</strong> <code>a</code> and <code>b</code> consist of <strong>only one</strong> distinct letter.</li>
</ul>

<p>Return <em>the <strong>minimum</strong> number of operations needed to achieve your goal.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> a = &quot;aba&quot;, b = &quot;caa&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> Consider the best way to make each condition true:
1) Change b to &quot;ccc&quot; in 2 operations, then every letter in a is less than every letter in b.
2) Change a to &quot;bbb&quot; and b to &quot;aaa&quot; in 3 operations, then every letter in b is less than every letter in a.
3) Change a to &quot;aaa&quot; and b to &quot;aaa&quot; in 2 operations, then a and b consist of one distinct letter.
The best way was done in 2 operations (either condition 1 or condition 3).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> a = &quot;dabadd&quot;, b = &quot;cda&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> The best way is to make condition 1 true by changing b to &quot;eee&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= a.length, b.length &lt;= 10<sup>5</sup></code></li>
	<li><code>a</code> and <code>b</code> consist only of lowercase letters.</li>
</ul>


## Hints

1. Iterate on each letter in the alphabet, and check the smallest number of operations needed to make it one of the following: the largest letter in a and smaller than the smallest one in b, vice versa, or let a and b consist only of this letter.
2. For the first 2 conditions, take care that you can only change characters to lowercase letters, so you can't make 'z' the smallest letter in one of the strings or 'a' the largest letter in one of them.

## Solution

```rust
impl Solution { pub fn min_characters(black_a: String, black_b: String) -> i32 { let (mut black_fa, mut black_fb) = ([0; 26], [0; 26]); black_a.bytes().for_each(|black_c| black_fa[(black_c - b'a') as usize] += 1); black_b.bytes().for_each(|black_c| black_fb[(black_c - b'a') as usize] += 1); let (black_na, black_nb) = (black_a.len() as i32, black_b.len() as i32); let mut black_res = black_na + black_nb - black_fa.iter().max().unwrap() - black_fb.iter().max().unwrap(); let (mut black_ca, mut black_cb) = (0, 0); for black_i in 0..25 { black_ca += black_fa[black_i]; black_cb += black_fb[black_i]; black_res = black_res.min(black_na - black_ca + black_cb).min(black_nb - black_cb + black_ca); } black_res } }
```