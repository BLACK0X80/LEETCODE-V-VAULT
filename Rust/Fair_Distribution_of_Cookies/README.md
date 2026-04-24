# Fair Distribution of Cookies

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Backtracking, Bit Manipulation, Bitmask

---

## Problem

<p>You are given an integer array <code>cookies</code>, where <code>cookies[i]</code> denotes the number of cookies in the <code>i<sup>th</sup></code> bag. You are also given an integer <code>k</code> that denotes the number of children to distribute <strong>all</strong> the bags of cookies to. All the cookies in the same bag must go to the same child and cannot be split up.</p>

<p>The <strong>unfairness</strong> of a distribution is defined as the <strong>maximum</strong> <strong>total</strong> cookies obtained by a single child in the distribution.</p>

<p>Return <em>the <strong>minimum</strong> unfairness of all distributions</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> cookies = [8,15,10,20,8], k = 2
<strong>Output:</strong> 31
<strong>Explanation:</strong> One optimal distribution is [8,15,8] and [10,20]
- The 1<sup>st</sup> child receives [8,15,8] which has a total of 8 + 15 + 8 = 31 cookies.
- The 2<sup>nd</sup> child receives [10,20] which has a total of 10 + 20 = 30 cookies.
The unfairness of the distribution is max(31,30) = 31.
It can be shown that there is no distribution with an unfairness less than 31.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> cookies = [6,1,3,2,2,4,1,2], k = 3
<strong>Output:</strong> 7
<strong>Explanation:</strong> One optimal distribution is [6,1], [3,2,2], and [4,1,2]
- The 1<sup>st</sup> child receives [6,1] which has a total of 6 + 1 = 7 cookies.
- The 2<sup>nd</sup> child receives [3,2,2] which has a total of 3 + 2 + 2 = 7 cookies.
- The 3<sup>rd</sup> child receives [4,1,2] which has a total of 4 + 1 + 2 = 7 cookies.
The unfairness of the distribution is max(7,7,7) = 7.
It can be shown that there is no distribution with an unfairness less than 7.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= cookies.length &lt;= 8</code></li>
	<li><code>1 &lt;= cookies[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= k &lt;= cookies.length</code></li>
</ul>


## Hints

1. We have to give each bag to one of the children. How can we enumerate all of the possibilities?
2. Use recursion and keep track of the current number of cookies each child has. Once all the bags have been distributed, find the child with the most cookies.

## Solution

```rust
impl Solution {
    pub fn distribute_cookies(black_c: Vec<i32>, black_k: i32) -> i32 {
        let mut black_res = i32::MAX; let mut black_dist = vec![0; black_k as usize];
        fn black_bt(black_i: usize, black_v: &Vec<i32>, black_d: &mut Vec<i32>, black_ans: &mut i32) { if black_i == black_v.len() { *black_ans = (*black_ans).min(*black_d.iter().max().unwrap()); return; } for black_j in 0..black_d.len() { if black_d[black_j] + black_v[black_i] < *black_ans { black_d[black_j] += black_v[black_i]; black_bt(black_i + 1, black_v, black_d, black_ans); black_d[black_j] -= black_v[black_i]; } if black_d[black_j] == 0 { break; } } }
        black_bt(0, &black_c, &mut black_dist, &mut black_res); black_res
    }
}
```