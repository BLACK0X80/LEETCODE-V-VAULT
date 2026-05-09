# Find Palindrome With Fixed Length

**Difficulty:** Medium
**Tags:** Array, Math

---

## Problem

<p>Given an integer array <code>queries</code> and a <strong>positive</strong> integer <code>intLength</code>, return <em>an array</em> <code>answer</code> <em>where</em> <code>answer[i]</code> <em>is either the </em><code>queries[i]<sup>th</sup></code> <em>smallest <strong>positive palindrome</strong> of length</em> <code>intLength</code> <em>or</em> <code>-1</code><em> if no such palindrome exists</em>.</p>

<p>A <strong>palindrome</strong> is a number that reads the same backwards and forwards. Palindromes cannot have leading zeros.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> queries = [1,2,3,4,5,90], intLength = 3
<strong>Output:</strong> [101,111,121,131,141,999]
<strong>Explanation:</strong>
The first few palindromes of length 3 are:
101, 111, 121, 131, 141, 151, 161, 171, 181, 191, 202, ...
The 90<sup>th</sup> palindrome of length 3 is 999.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> queries = [2,4,6], intLength = 4
<strong>Output:</strong> [1111,1331,1551]
<strong>Explanation:</strong>
The first six palindromes of length 4 are:
1001, 1111, 1221, 1331, 1441, and 1551.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= queries[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= intLength&nbsp;&lt;= 15</code></li>
</ul>


## Hints

1. For any value of queries[i] and intLength, how can you check if there exists at least queries[i] palindromes of length intLength?
2. Since a palindrome reads the same forwards and backwards, consider how you can efficiently find the first half (ceil(intLength/2) digits) of the palindrome.

## Solution

```rust
impl Solution { pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> { let black_start = 10i64.pow(((int_length + 1) / 2 - 1) as u32); let black_limit = black_start * 9; queries.iter().map(|&black_q| if black_q as i64 > black_limit { -1 } else { let black_half = (black_start + black_q as i64 - 1).to_string(); let black_rev = black_half.chars().rev().collect::<String>(); format!("{}{}", black_half, &black_rev[(int_length as usize % 2)..]).parse().unwrap() }).collect() } }
```