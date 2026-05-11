# Largest Palindromic Number

**Difficulty:** Medium
**Tags:** Hash Table, String, Greedy, Counting

---

## Problem

<p>You are given a string <code>num</code> consisting of digits only.</p>

<p>Return <em>the <strong>largest palindromic</strong> integer (in the form of a string) that can be formed using digits taken from </em><code>num</code>. It should not contain <strong>leading zeroes</strong>.</p>

<p><strong>Notes:</strong></p>

<ul>
	<li>You do <strong>not</strong> need to use all the digits of <code>num</code>, but you must use <strong>at least</strong> one digit.</li>
	<li>The digits can be reordered.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;444947137&quot;
<strong>Output:</strong> &quot;7449447&quot;
<strong>Explanation:</strong> 
Use the digits &quot;4449477&quot; from &quot;<u><strong>44494</strong></u><u><strong>7</strong></u>13<u><strong>7</strong></u>&quot; to form the palindromic integer &quot;7449447&quot;.
It can be shown that &quot;7449447&quot; is the largest palindromic integer that can be formed.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;00009&quot;
<strong>Output:</strong> &quot;9&quot;
<strong>Explanation:</strong> 
It can be shown that &quot;9&quot; is the largest palindromic integer that can be formed.
Note that the integer returned should not contain leading zeroes.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 10<sup>5</sup></code></li>
	<li><code>num</code> consists of digits.</li>
</ul>


## Hints

1. In order to form a valid palindrome, other than the middle digit in an odd-length palindrome, every digit needs to exist on both sides.
2. A longer palindrome implies a larger valued palindrome. For palindromes of the same length, the larger digits should occur first.
3. We can count the occurrences of each digit and build the palindrome starting from the ends. Starting from the larger digits, if there are still at least 2 occurrences of a digit, we can place these digits on each side.
4. Make sure to consider the special case for the center digit (if any) and zeroes. There should not be leading zeroes.

## Solution

```rust
impl Solution { pub fn largest_palindromic(black_num: String) -> String { let mut black_cnt = [0i32; 10]; for b in black_num.bytes() { black_cnt[(b - b'0') as usize] += 1; } let (mut black_half, mut black_mid) = (String::new(), None); for i in (0..=9).rev() { if black_cnt[i] % 2 == 1 && black_mid.is_none() { black_mid = Some((i as u8 + b'0') as char); } for _ in 0..(black_cnt[i] / 2) { if i == 0 && black_half.is_empty() { continue; } black_half.push((i as u8 + b'0') as char); } } if black_half.is_empty() && black_mid.is_none() { return "0".to_string(); } let mut black_res = black_half.clone(); if let Some(m) = black_mid { black_res.push(m); } black_res.extend(black_half.chars().rev()); black_res } }
```